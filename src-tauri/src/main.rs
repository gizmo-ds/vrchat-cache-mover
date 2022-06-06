#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

const WEBVIEW2_DOWNLOAD_URL: &str =
  "https://developer.microsoft.com/microsoft-edge/webview2#download-section";

mod vrchat;

fn main() {
  match tauri::Builder::default()
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
  {
    Ok(_) => (),
    Err(e) => {
      if e.to_string().contains("WebView2") {
        std::process::Command::new("cmd.exe")
          .args(&["/C", "start", WEBVIEW2_DOWNLOAD_URL])
          .spawn()
          .unwrap();
      }
      std::process::exit(1);
    }
  }
}
