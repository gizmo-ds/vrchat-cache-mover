#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

mod vrchat;

const WEBVIEW2_DOWNLOAD_URL: &str =
  "https://developer.microsoft.com/microsoft-edge/webview2#download-section";

fn main() {
  if let Err(err) = tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      vrchat::vrchat_path,
      vrchat::vrchat_config,
      vrchat::save_config,
      vrchat::total_cache,
      vrchat::move_cache,
      vrchat::remove_cache,
      vrchat::open_vrchat_path,
      vrchat::check_new_path,
      ui_created,
    ])
    .run(tauri::generate_context!())
  {
    if err.to_string().contains("WebView2") {
      std::process::Command::new("cmd.exe")
        .args(&["/C", "start", WEBVIEW2_DOWNLOAD_URL])
        .spawn()
        .unwrap();
    }
    std::process::exit(1);
  }
}

#[tauri::command]
async fn ui_created(window: tauri::Window) {
  window.get_window("main").unwrap().show().unwrap();
}
