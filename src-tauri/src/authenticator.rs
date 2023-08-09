// Microsoft Device Code Flow : https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
// Pretty much everything else : https://wiki.vg/Microsoft_Authentication_Scheme

use std::{
    println, 
    format, 
    collections::HashMap, 
    fs::{self, File},
    path::PathBuf, io::BufReader,
};

use reqwest::StatusCode;
use serde::{
    Serialize, 
    Deserialize
};
use serde_json::json;
use tauri::Window;

const CLIENT_ID: &str = "c1e288f4-4793-4bfd-bb9e-e3ea3e14218e";

#[derive(Debug, Serialize, Deserialize)]
struct DeviceAuthorizationRequest
{
    client_id: String,
    scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceAuthorizationResponse
{
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: i32,
    interval: i32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAuthenticationRequest
{
    grant_type: String,
    client_id: String,
    device_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserRefreshRequest
{
    grant_type: String,
    client_id: String,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAuthenticationResponse
{
    token_type: String,
    scope: String,
    expires_in: i32,
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct XboxAuthenticationResponse
{
    issue_instant: String,
    not_after: String,
    token: String,
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MinecraftXSTSResponse
{
    issue_instant: String,
    not_after: String,
    token: String,
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MinecraftAuthenticationResponse
{
    username: String,
    access_token: String,
    token_type: String,
    expires_in: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MinecraftProfile
{
    id: String,
    name: String,
    skins: Vec<HashMap<String, String>>,
    capes: Vec<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResult {
    pub access_token: String,
    pub uuid: String,
    pub username: String,
}

pub struct Authenticator {
    pub profile: AuthResult,
}

impl Authenticator {
    pub async fn new(game_dir: &PathBuf, window: &Window) -> Self {
        let profile = authenticate(&game_dir, &window)
            .await
            .expect("Unable to authenticate");

        Self {
            profile,
        }
    }
}

pub async fn authenticate(game_dir: &PathBuf, window: &Window) -> Result<AuthResult, Box<dyn std::error::Error>> {
    window.emit("launcher-log", "Authentification en cours");
    // Only one client needed
    let client = reqwest::Client::new();

    // Checking for saved credentials
    let mut token_path = PathBuf::from(&game_dir);
    token_path.push("token.json");

    if token_path.exists() {
        let file = File::open(&token_path)?;
        let buffer = BufReader::new(file);
        let saved_resp: UserAuthenticationResponse = serde_json::from_reader(buffer).expect("Failed to read saved credentials");

        let user_refresh_req = UserRefreshRequest {
            grant_type: "refresh_token".into(),
            client_id: CLIENT_ID.into(),
            refresh_token: saved_resp.refresh_token.into(),
        };

        let user_refresh_resp: UserAuthenticationResponse = 
            client.post(format!("https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token",
                                tenant = "consumers"))
            .form(&user_refresh_req)
            .send()
            .await?
            .json()
            .await?;

        fs::write(&token_path, serde_json::to_string(&user_refresh_resp)
                  .expect("Failed to save authentication token"))?;

    } else {

        // MICROSOFT AUTHENTICATION
        // Device Authorization
        let dev_auth_req = DeviceAuthorizationRequest {
            client_id: CLIENT_ID.into(),
            scope: "XboxLive.signin offline_access".into()
        };

        println!("Connecting to Microsoft...");

        //println!("{}", serde_json::to_string_pretty(&dev_auth_req)?);

        let device_auth_resp: DeviceAuthorizationResponse = 
            client.post(format!("https://login.microsoftonline.com/{tenant}/oauth2/v2.0/devicecode",
                                tenant = "consumers"))
            .form(&dev_auth_req)
            .send()
            .await?
            .json()
            .await?;

        //println!("{}", serde_json::to_string_pretty(&device_auth_resp)?);

        window.emit("launcher-log", format!("{}", &device_auth_resp.message));

        // User Authentication
        let user_auth_req = UserAuthenticationRequest {
            grant_type: "urn:ietf:params:oauth:grant-type:device_code".into(),
            client_id: CLIENT_ID.into(),
            device_code: device_auth_resp.device_code.into(),
        };

        //println!("{}", serde_json::to_string_pretty(&user_auth_req)?);

        // Polling until the user authenticates
        let user_auth_resp: UserAuthenticationResponse = loop {
            std::thread::sleep(std::time::Duration::from_secs(3));

            let response = 
                client.post(format!("https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token",
                                    tenant = "consumers"))
                .form(&user_auth_req)
                .send()
                .await?;

            if response.status() == StatusCode::OK {
                window.emit("launcher-log", "Authentification en cours");
                break response.json()
                    .await?;

            }
        };

        fs::write(&token_path, serde_json::to_string(&user_auth_resp)
                  .expect("Failed to save authentication token"))?;
        //println!("{}", serde_json::to_string_pretty(&user_auth_resp)?);
    }

    let file = File::open(&token_path)?;
    let buffer = BufReader::new(file);
    let saved_resp: UserAuthenticationResponse = serde_json::from_reader(buffer).expect("Failed to read saved credentials");

    // XBOXLIVE AUTHENTICATION
    let xbox_auth_req = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={}", &saved_resp.access_token) 
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });

    let xbox_auth_resp: XboxAuthenticationResponse = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&xbox_auth_req)
        .send()
        .await?
        .json()
        .await?;

    println!("Logging in Xbox Live...");

    //println!("{}", serde_json::to_string_pretty(&xbox_auth_resp)?);

    // MINECRAFT XSTS TOKEN

    let minecraft_xsts_req = json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [&xbox_auth_resp.token]
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT"
    });

    let minecraft_xsts_resp: MinecraftXSTSResponse = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&minecraft_xsts_req)
        .send()
        .await?
        .json()
        .await?;

    println!("Fetching Minecraft XSTS Token...");

    //println!("{}", serde_json::to_string_pretty(&minecraft_xsts_resp)?);

    // MINECRAFT AUTHENTICATION
    let minecraft_auth_req = json!({
        "identityToken": format!("XBL3.0 x={userhash};{xsts_token}",
                                 userhash = &minecraft_xsts_resp.display_claims["xui"][0]["uhs"], 
                                 xsts_token = &minecraft_xsts_resp.token),
                                 "ensureLegacyEnabled" : true
    });

    //println!("{}", serde_json::to_string_pretty(&minecraft_auth_req)?);

    let minecraft_auth_resp: MinecraftAuthenticationResponse = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&minecraft_auth_req)
        .send()
        .await?
        .json()
        .await?;

    println!("Logging in Minecraft Services...");

    //println!("{}", serde_json::to_string_pretty(&minecraft_auth_resp)?);

    // GAME OWNERSHIP
    // TODO
    // ---

    let minecraft_profile: MinecraftProfile = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(&minecraft_auth_resp.access_token)
        .send()
        .await?
        .json()
        .await?;

    println!("Fetching Minecraft Profile...");
    //println!("{}", serde_json::to_string_pretty(&minecraft_profile)?);

    let result = AuthResult {
        access_token: minecraft_auth_resp.access_token,
        uuid: minecraft_profile.id,
        username: minecraft_profile.name,
    };

    println!("Connected!");
    //println!("{:?}", result);

    let mut profile_path = PathBuf::from(&game_dir);
    profile_path.push("profile.json");
    fs::write(profile_path, serde_json::to_string(&result)
              .expect("Failed to save authentication data"))?;

    Ok(result)
}
