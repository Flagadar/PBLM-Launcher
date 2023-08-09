use std::{
    path::{Path, PathBuf}, 
    println
};

use serde_json::Value;
use tauri::Window;

use crate::{fabric, mods};

use self::{
    version_manifest::VersionManifest, 
    version_file::VersionFile
};

use super::downloader::Downloader;

mod version_manifest;
mod version_file;

const VERSION_MANIFEST: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

// NATIVES  
pub struct Installer {
    pub game: GameInfo,
    pub sys: SysInfo,
    pub downloader: Downloader,
}

pub struct GameInfo {
    pub minecraft_version: String,
}

pub struct SysInfo {
    pub os: String,
    pub game_dir: PathBuf,
}

impl Installer {
    pub fn new(minecraft_version: &str) -> Self {
        let downloader = Downloader::new();
        let os_type = std::env::consts::OS;

        let home_dir = match os_type {
            "linux" => dirs::home_dir().expect("Failed to locate home directory"),
            "macos" => dirs::home_dir().expect("Failed to locate home directory"),
            "windows" => dirs::home_dir().expect("Failed to locate AppData"),
            _ => panic!("Unsupported OS")
        };
        
        let game_dir = Path::new(&home_dir).join(".pblm/");
        if !game_dir.exists() {
            match std::fs::create_dir(&game_dir) {
                Ok(path) => {
                    println!("Successfully created game directory");
                    path
                },
                Err(err) => println!("Unable to create game directory : {}", err),
            };
        }

        let sys = SysInfo {
            os: os_type.to_string(),
            game_dir,
        };

        let game = GameInfo {
            minecraft_version: minecraft_version.to_string(),
        };

        Self { 
            game, 
            sys, 
            downloader,
        }
    }

    pub async fn install(&mut self, window: &Window) -> Result<(), Box<dyn std::error::Error>> {
            window.emit("launcher-log", "Installation en cours...")
            .expect("Failed to log");

        let version_manifest: VersionManifest = serde_json::from_value(
            self.downloader.to_json(VERSION_MANIFEST.to_string())
            .await
            .expect("Failed to fetch version manifest"))
            .expect("Failed to convert version manifest");

        let version_file = version_manifest
            .versions
            .into_iter()
            .find(|ver| ver.id == self.game.minecraft_version)
            .expect("Unable to parse version manifest");

        let version_file: VersionFile = serde_json::from_value(self.downloader
            .to_json(version_file.url)
            .await
            .expect("Failed to fetch version file"))
            .expect("Failed to convert version file");
        
        match self.install_assets(&version_file).await {
            Ok(_) => println!("Assets added to queue successfully"),
            Err(err) => println!("Error while adding assets to queue : {}", err),
        };

        match self.install_libraries(&version_file) {
            Ok(_) => println!("Libraries added to queue successfully"),
            Err(err) => println!("Error while adding libraries to queue : {}", err),
        };
           
        match self.install_client(&version_file) {
            Ok(_) => println!("Client added to queue successfully"),
            Err(err) => println!("Error while adding client to queue : {}", err),
        };

        match self.install_fabric() {
            Ok(_) => println!("Fabric added to queue successfully"),
            Err(err) => println!("Error while adding Fabric to queue : {}", err),
        };

        match self.install_mods() {
            Ok(_) => println!("Mods added to queue successfully"),
            Err(err) => println!("Error while adding mods to queue : {}", err),
        };
        self.downloader.download_queue().await;

            window.emit("launcher-log", "Installation terminée")
            .expect("Failed to log");

        Ok(())
    }

    async fn install_assets(&mut self, version_file: &VersionFile) -> Result<(), Box<dyn std::error::Error>> {
        let mut index_path = PathBuf::from(&self.sys.game_dir);
        index_path.push("assets/indexes");
        self.downloader.push_to_queue(version_file.asset_index.url.clone(), index_path)
            .expect("Failed to push asset index to queue");
        
        let asset_index: Value = serde_json::from_value(
            self.downloader.to_json(version_file.asset_index.url.clone())
            .await
            .expect("Failed to fetch asset index"))
            .expect("Failed to convert asset index");

        let assets = asset_index["objects"].as_object()
            .expect("Failed to parse objects in the asset index");

        for asset in assets {
            let mut path = PathBuf::new();
            let hash = asset.1["hash"].as_str()
                .expect("Failed to parse hash from asset");
            path.push(&self.sys.game_dir);
            path.push(format!("assets/objects/{}", hash.get(0..2)
                              .expect("Failed to name an asset")));
            self.downloader.push_to_queue(format!(
                    "https://resources.download.minecraft.net/{}/{}", 
                    hash.get(0..2)
                    .expect("Failed to name asset dir"),
                    hash),
                    path)
                .expect("Failed to push asset to queue");
        }
        Ok(())
    }

    fn install_client(&mut self, version_file: &VersionFile) -> Result<(), Box<dyn std::error::Error>> {
        let mut path = PathBuf::new();
        path.push(&self.sys.game_dir);
        path.push("client");
        self.downloader.push_to_queue(version_file.downloads.client.url.clone(), path)
            .expect("Failed to push client to queue");

        Ok(())
    }

    fn install_libraries(&mut self, version_file: &VersionFile) -> Result<(), Box<dyn std::error::Error>> {
        for lib in &version_file.libraries {
            let mut path = PathBuf::new();
            path.push(&self.sys.game_dir);
            match lib.rules.get(0) {
                Some(rule) => match rule.os.name.as_str() {
                    "linux" => path.push("natives/linux"),
                    "osx" => path.push("natives/osx"),
                    "windows" => path.push("natives/windows"),
                    _ => println!("Failed to parse libraries"),
                },
                None => path.push("libraries"),
            }
            self.downloader.push_to_queue(lib.downloads.artifact.url.clone(), path)
                .expect("Failed to push library to queue");
        }
        Ok(())
    }

    fn install_fabric(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let fabric_libs = fabric::get_fabric_libs();

        for lib in fabric_libs {
            let mut path = PathBuf::from(&self.sys.game_dir);
            path.push("libraries/fabric");
            self.downloader.push_to_queue(lib, path)
                .expect("Failed to download Fabric");
        }
        Ok(())
    }

    fn install_mods(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mods = mods::get_mod_list();

        for modif in mods {
            let mut path = PathBuf::from(&self.sys.game_dir);
            path.push("mods");
            self.downloader.push_to_queue(modif, path)
                .expect("Failed to download mods");
        }
        Ok(())
    }
}
