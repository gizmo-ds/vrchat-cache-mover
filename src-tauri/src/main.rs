#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod vrchat;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            vrchat::vrchat_path,
            vrchat::vrchat_config,
            vrchat::save_config,
            vrchat::total_cache,
            vrchat::move_cache,
            vrchat::remove_cache,
            vrchat::open_vrchat_path,
            vrchat::check_new_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
