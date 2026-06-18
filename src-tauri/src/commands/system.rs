//! System / platform / path commands. Directory getters return real paths via
//! the Tauri path resolver (with a canned fallback); platform flags and
//! accelerators return canned values matching the mock; `open_*` and lifecycle
//! commands are logged no-ops.

use tauri::{AppHandle, Manager};

use crate::error::AppResult;
use crate::types::AvailableAccelerators;

#[tauri::command]
pub fn get_app_dir_path(app: AppHandle) -> AppResult<String> {
    log::info!("[cmd] get_app_dir_path");
    let path = app
        .path()
        .app_data_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "/Users/mock/aivision/data".to_string());
    Ok(path)
}

#[tauri::command]
pub fn get_log_dir_path(app: AppHandle) -> AppResult<String> {
    log::info!("[cmd] get_log_dir_path");
    let path = app
        .path()
        .app_log_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "/Users/mock/aivision/logs".to_string());
    Ok(path)
}

#[tauri::command]
pub fn open_recordings_folder() -> AppResult<()> {
    log::info!("[cmd] open_recordings_folder");
    Ok(())
}

#[tauri::command]
pub fn open_log_dir() -> AppResult<()> {
    log::info!("[cmd] open_log_dir");
    Ok(())
}

#[tauri::command]
pub fn open_app_data_dir() -> AppResult<()> {
    log::info!("[cmd] open_app_data_dir");
    Ok(())
}

/// Bare return (`Promise<boolean>`).
#[tauri::command]
pub fn is_portable() -> bool {
    log::info!("[cmd] is_portable");
    false
}

#[tauri::command]
pub fn is_laptop() -> AppResult<bool> {
    log::info!("[cmd] is_laptop");
    Ok(true)
}

/// Bare return (`Promise<boolean>`).
#[tauri::command]
pub fn is_recording() -> bool {
    log::info!("[cmd] is_recording");
    false
}

/// Bare return (`Promise<boolean>`).
#[tauri::command]
pub fn check_apple_intelligence_available() -> bool {
    log::info!("[cmd] check_apple_intelligence_available");
    false
}

/// Bare return (`Promise<AvailableAccelerators>`).
#[tauri::command]
pub fn get_available_accelerators() -> AvailableAccelerators {
    log::info!("[cmd] get_available_accelerators");
    AvailableAccelerators {
        whisper: vec!["auto".to_string(), "cpu".to_string(), "gpu".to_string()],
        ort: vec!["auto".to_string(), "cpu".to_string()],
        gpu_devices: Vec::new(),
    }
}

/// Bare return (`Promise<void>`).
#[tauri::command]
pub fn cancel_operation() {
    log::info!("[cmd] cancel_operation");
}

#[tauri::command]
pub fn trigger_update_check() -> AppResult<()> {
    log::info!("[cmd] trigger_update_check");
    Ok(())
}

#[tauri::command]
pub fn show_main_window_command() -> AppResult<()> {
    log::info!("[cmd] show_main_window_command");
    Ok(())
}
