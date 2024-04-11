// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::set_window_shadow;
mod fs_api;
mod utils;

use tauri::{App, Manager};
use window_vibrancy::{self, NSVisualEffectMaterial};

/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_window("main").unwrap();

    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // 仅在 windows 下执行
    #[cfg(target_os = "windows")]
    window_vibrancy::apply_mica(&win, Some(true))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    set_window_shadow(app);

    Ok(())
}


fn main() {
    tauri::Builder::default()
        .setup(init)
        .invoke_handler(tauri::generate_handler![fs_api::rename_dir]) /* add event handlers here */
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
