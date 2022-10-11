use bytesize::ByteSize;
use path_absolutize::*;
use std::fs;
use std::{env, path::Path};

#[tauri::command]
#[cfg(target_os = "windows")]
pub fn vrchat_path() -> Option<String> {
  let _vrchat_path = match env::var("LOCALAPPDATA").or_else(|_| env::var("APPDATA")) {
    Ok(path) => Path::new(path.as_str()).join("..//LocalLow//VRChat//VRChat"),
    Err(_) => return None,
  };
  Some(
    _vrchat_path
      .absolutize()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string(),
  )
}

#[tauri::command]
pub fn check_vrchat_path() -> Result<(), tauri::InvokeError> {
  if let Some(_vrchat_path) = vrchat_path() {
    if !Path::new(_vrchat_path.as_str()).exists() {
      return Err(tauri::InvokeError::from("vrchat-path-notfound"));
    }
  }
  Ok(())
}

#[tauri::command]
pub fn vrchat_config() -> Result<String, tauri::InvokeError> {
  if let Some(_vrchat_path) = vrchat_path() {
    let config_path = Path::new(_vrchat_path.as_str()).join("config.json");
    if !config_path.exists() {
      return Ok("{}".to_string());
    }
    return match fs::read_to_string(config_path) {
      Ok(config) => Ok(config),
      Err(err) => Err(tauri::InvokeError::from(err.to_string())),
    };
  }
  Err(tauri::InvokeError::from("vrchat-path-notfound"))
}

#[tauri::command]
pub fn total_cache() -> Option<String> {
  if let Some(_vrchat_path) = vrchat_path() {
    let cache_path = Path::new(_vrchat_path.as_str()).join("Cache-WindowsPlayer");
    let total_size = fs_extra::dir::get_size(cache_path).ok()?;
    return Some(ByteSize::b(total_size).to_string_as(true));
  }
  None
}

#[tauri::command]
pub async fn move_cache(new_path: &str) -> Result<(), tauri::InvokeError> {
  match vrchat_path() {
    Some(_vrchat_path) => {
      let cache_path = Path::new(_vrchat_path.as_str()).join("Cache-WindowsPlayer");
      if !cache_path.exists() {
        return Err(tauri::InvokeError::from("cache-directory-notfound"));
      }
      if let Err(err) = check_new_path(new_path) {
        return Err(err);
      }
      let _new_path = new_path.to_string();
      if let Err(err) =
        fs_extra::dir::move_dir(cache_path, _new_path, &fs_extra::dir::CopyOptions::new())
      {
        return Err(tauri::InvokeError::from(err.to_string()));
      }
      Ok(())
    }
    None => Err(tauri::InvokeError::from("vrchat-path-notfound")),
  }
}

#[tauri::command]
pub fn remove_cache() {
  if let Some(_vrchat_path) = vrchat_path() {
    let cache_path = Path::new(_vrchat_path.as_str()).join("Cache-WindowsPlayer");
    if cache_path.exists() {
      fs_extra::dir::remove(cache_path).ok();
    }
  }
}

#[tauri::command]
#[cfg(target_os = "windows")]
pub fn open_vrchat_path() {
  use std::os::windows::process::CommandExt;
  use std::process::Command;

  if let Some(_vrchat_path) = vrchat_path() {
    Command::new("cmd")
      .creation_flags(0x08)
      .args(&["/C", "start", _vrchat_path.as_str()])
      .spawn()
      .ok();
  }
}

#[tauri::command]
pub fn save_config(config: &str) -> Result<(), tauri::InvokeError> {
  if let Some(_vrchat_path) = vrchat_path() {
    let config_path = Path::new(_vrchat_path.as_str()).join("config.json");
    fs::write(config_path, config).ok();
    return Ok(());
  }
  Err(tauri::InvokeError::from("vrchat-path-notfound"))
}

#[tauri::command]
pub fn check_new_path(new_path: &str) -> Result<(), tauri::InvokeError> {
  let _new_path = Path::new(new_path);
  if !_new_path.exists() {
    return Err(tauri::InvokeError::from("target-directory-not-exist"));
  } else if !_new_path.is_dir() {
    return Err(tauri::InvokeError::from("target-directory-not-directory"));
  }
  return Ok(());
}
