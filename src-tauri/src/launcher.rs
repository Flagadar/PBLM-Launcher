use std::{fs, io::Read};

use walkdir::WalkDir;

use super::{authenticator::{AuthResult, Authenticator}, installer::Installer};


pub struct Launcher {
    pub auth: Authenticator,
    pub installer: Installer,
    pub args: Vec<String>,
}

impl Launcher {
    pub async fn new(auth: Authenticator, installer: Installer) -> Self {
        let game_dir = String::from(installer.sys.game_dir
                                    .to_str()
                                    .expect("Unable to parse game directory"));

        let mut assets_dir = String::from(&game_dir);
        assets_dir.push_str("assets");

        let mut natives_path = String::from(&game_dir);
        match std::env::consts::OS {
            "linux" => natives_path.push_str("natives/linux"),
            "macos" => natives_path.push_str("natives/osx"),
            "windows" => natives_path.push_str("natives/windows"),
            _ => panic!("Unsupported OS"),
        };

        let mut client_path = String::from(&game_dir);
        //if installer.game.mod_loader == true {
        //    client_path.push_str("/versions");
        //} else {
            client_path.push_str("client");
        //}

        let mut lib_path = String::from(&game_dir);
        lib_path.push_str("libraries");

        let mut classpath_arg = String::new();

        let win_separator = String::from(';');
        let lin_separator = String::from(':');

        for file in WalkDir::new(&client_path) {
            classpath_arg.push_str(file
                                   .expect("Failed to extract file name")
                                   .path()
                                   .to_str()
                                   .expect("Failed to convert file name to str"));
            match std::env::consts::OS {
                "linux" => classpath_arg.push_str(&lin_separator),
                "macos" => classpath_arg.push_str(&lin_separator),
                "windows" => classpath_arg.push_str(&win_separator),
                _ => panic!("Unsupported OS"),
           }
        
        }

        for file in WalkDir::new(&natives_path) {
            classpath_arg.push_str(file
                                   .expect("Failed to extract file name")
                                   .path()
                                   .to_str()
                                   .expect("Failed to convert file name to str"));
            match std::env::consts::OS {
                "linux" => classpath_arg.push_str(&lin_separator),
                "macos" => classpath_arg.push_str(&lin_separator),
                "windows" => classpath_arg.push_str(&win_separator),
                _ => panic!("Unsupported OS"),
            }

        }

        for file in WalkDir::new(&lib_path) {
            classpath_arg.push_str(file
                                   .expect("Failed to extract file name")
                                   .path()
                                   .to_str()
                                   .expect("Failed to convert file name to str"));
            match std::env::consts::OS {
                "linux" => classpath_arg.push_str(&lin_separator),
                "macos" => classpath_arg.push_str(&lin_separator),
                "windows" => classpath_arg.push_str(&win_separator),
                _ => panic!("Unsupported OS"),
            }
        }

        //println!("{}", &classpath_arg);
        let mut profile_path = String::from(&game_dir);
        profile_path.push_str("profile.json");
        let mut file = fs::File::open(&profile_path)
            .expect("Failed to open player profile");
        let mut file_str = String::new();
        file.read_to_string(&mut file_str)
            .expect("Failed to read player profile");

        let profile: AuthResult = serde_json::from_str(&file_str)
            .expect("Failed to convert profile");

        let mut launch_args: Vec<String> = Vec::new();
        launch_args.extend_from_slice(&[
                                      format!("-Djava.library.path={}", natives_path),
                                      format!("-Dminecraft.launcher.brand=PBLM"),
                                      format!("-Dminecraft.launcher.version=0.1"),
                                      format!("-cp"),
                                      classpath_arg, 
                                      format!("net.fabricmc.loader.impl.launch.knot.KnotClient"),
                                      format!("-DFabricMcEmu=net.minecraft.client.main.Main"),
                                      format!("-Xmx6G"),
                                      format!("-XX:+UnlockExperimentalVMOptions"),
                                      format!("-XX:+UseG1GC"),
                                      format!("-XX:G1NewSizePercent=20"),
                                      format!("-XX:G1ReservePercent=20"),
                                      format!("-XX:MaxGCPauseMillis=50"),
                                      format!("-XX:G1HeapRegionSize=32M"),
                                      format!("--username"),
                                      profile.username,
                                      format!("--version"),
                                      format!("1.19.2"),
                                      format!("--gameDir"),
                                      game_dir,
                                      format!("--assetsDir"),
                                      assets_dir,
                                      format!("--assetIndex"),
                                      format!("1.19"),
                                      format!("--uuid"),
                                      profile.uuid,
                                      format!("--accessToken"),
                                      profile.access_token,
                                      format!("--userType"),
                                      format!("msa"),
                                      format!("--versionType"),
                                      format!("release"),
                                      ]);

        std::process::Command::new("java")
            .args(&launch_args)
            .status()
            .expect("Failed to launch the game");

        Self { 
            auth, 
            installer, 
            args: launch_args
        }
    }
}
