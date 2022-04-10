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
pub fn vrchat_config() -> Result<String, tauri::InvokeError> {
    if let Some(_vrchat_path) = vrchat_path() {
        let config_path = Path::new(_vrchat_path.as_str()).join("config.json");
        return match fs::read_to_string(config_path) {
            Ok(config) => Ok(config),
            Err(_) => Err(tauri::InvokeError::from("Could not read config.json")),
        };
    }
    Err(tauri::InvokeError::from("Could not find vrchat path"))
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
pub fn move_cache(new_path: &str) -> Result<(), &str> {
    match vrchat_path() {
        Some(_vrchat_path) => {
            let cache_path = Path::new(_vrchat_path.as_str()).join("Cache-WindowsPlayer");
            if !cache_path.exists() {
                return Err("未找到旧的缓存目录");
            }
            let _new_path = Path::new(&new_path);
            if !_new_path.exists() {
                return Err("目标目录不存在");
            }
            let _new_path = _new_path.to_str().unwrap().to_string();
            std::thread::spawn(move || {
                fs_extra::dir::copy_with_progress(
                    cache_path,
                    _new_path,
                    &fs_extra::dir::CopyOptions::new(),
                    |process_info| {
                        println!("{}", process_info.total_bytes);
                        fs_extra::dir::TransitProcessResult::ContinueOrAbort
                    },
                )
                .unwrap();
            });
            Ok(())
        }
        None => Err("未找到VRChat配置目录"),
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
    if let Some(_vrchat_path) = vrchat_path() {
        std::process::Command::new("cmd.exe")
            .args(&["/C", "start", _vrchat_path.as_str()])
            .spawn()
            .ok();
    }
}

#[tauri::command]
pub fn save_config(config: &str) -> Result<(), &str> {
    if let Some(_vrchat_path) = vrchat_path() {
        let config_path = Path::new(_vrchat_path.as_str()).join("config.json");
        fs::write(config_path, config).ok();
        return Ok(());
    }
    Err("未找到VRChat配置目录")
}
