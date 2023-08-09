#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use launcher::Launcher;
use tauri::Window;

use authenticator::Authenticator;
use installer::Installer;

pub mod launcher;
pub mod installer;
pub mod downloader;
pub mod authenticator;
pub mod mods;
pub mod fabric;

#[tauri::command]
async fn launch(window: Window) {
    let mut installer = Installer::new("1.19.2");

    let auth = Authenticator::new(&installer.sys.game_dir, &window)
        .await;

    installer.install(&window)
        .await
        .expect("Unable to install the game");

    window.emit("launcher-log", "En cours de lancement").unwrap();
    let launcher = Launcher::new(auth, installer)
        .await;
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![launch])
    .run(tauri::generate_context!())
    .expect("Error while running tauri application");
}
