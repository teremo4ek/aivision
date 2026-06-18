//! Audio device commands. Device-selection setters lightly persist into
//! `AppSettings` so the matching getters stay consistent (a small upgrade over
//! the mock's no-op setters); test-sound and custom-sounds are stubbed.

use std::sync::Mutex;

use tauri::State;

use crate::error::AppResult;
use crate::state::AppState;
use crate::types::{AudioDevice, CustomSounds};

#[tauri::command]
pub fn get_available_microphones(
    state: State<'_, Mutex<AppState>>,
) -> AppResult<Vec<AudioDevice>> {
    log::info!("[cmd] get_available_microphones");
    Ok(state.lock().unwrap().microphones.clone())
}

#[tauri::command]
pub fn get_available_output_devices(
    state: State<'_, Mutex<AppState>>,
) -> AppResult<Vec<AudioDevice>> {
    log::info!("[cmd] get_available_output_devices");
    Ok(state.lock().unwrap().output_devices.clone())
}

#[tauri::command]
pub fn get_microphone_mode(state: State<'_, Mutex<AppState>>) -> AppResult<bool> {
    log::info!("[cmd] get_microphone_mode");
    Ok(state
        .lock()
        .unwrap()
        .settings
        .always_on_microphone
        .unwrap_or(false))
}

#[tauri::command]
pub fn update_microphone_mode(
    state: State<'_, Mutex<AppState>>,
    always_on: bool,
) -> AppResult<()> {
    log::info!("[cmd] update_microphone_mode always_on={always_on}");
    state.lock().unwrap().settings.always_on_microphone = Some(always_on);
    Ok(())
}

#[tauri::command]
pub fn get_selected_microphone(state: State<'_, Mutex<AppState>>) -> AppResult<String> {
    log::info!("[cmd] get_selected_microphone");
    Ok(state
        .lock()
        .unwrap()
        .settings
        .selected_microphone
        .clone()
        .unwrap_or_else(|| "default".to_string()))
}

#[tauri::command]
pub fn set_selected_microphone(
    state: State<'_, Mutex<AppState>>,
    device_name: String,
) -> AppResult<()> {
    log::info!("[cmd] set_selected_microphone device_name={device_name}");
    state.lock().unwrap().settings.selected_microphone = Some(device_name);
    Ok(())
}

#[tauri::command]
pub fn get_selected_output_device(state: State<'_, Mutex<AppState>>) -> AppResult<String> {
    log::info!("[cmd] get_selected_output_device");
    Ok(state
        .lock()
        .unwrap()
        .settings
        .selected_output_device
        .clone()
        .unwrap_or_else(|| "default".to_string()))
}

#[tauri::command]
pub fn set_selected_output_device(
    state: State<'_, Mutex<AppState>>,
    device_name: String,
) -> AppResult<()> {
    log::info!("[cmd] set_selected_output_device device_name={device_name}");
    state.lock().unwrap().settings.selected_output_device = Some(device_name);
    Ok(())
}

#[tauri::command]
pub fn get_clamshell_microphone(state: State<'_, Mutex<AppState>>) -> AppResult<String> {
    log::info!("[cmd] get_clamshell_microphone");
    Ok(state
        .lock()
        .unwrap()
        .settings
        .clamshell_microphone
        .clone()
        .unwrap_or_else(|| "default".to_string()))
}

#[tauri::command]
pub fn set_clamshell_microphone(
    state: State<'_, Mutex<AppState>>,
    device_name: String,
) -> AppResult<()> {
    log::info!("[cmd] set_clamshell_microphone device_name={device_name}");
    state.lock().unwrap().settings.clamshell_microphone = Some(device_name);
    Ok(())
}

/// Bare return (`Promise<void>`).
#[tauri::command]
pub fn play_test_sound(sound_type: String) {
    log::info!("[cmd] play_test_sound sound_type={sound_type}");
}

/// Bare return (`Promise<CustomSounds>`).
#[tauri::command]
pub fn check_custom_sounds() -> CustomSounds {
    log::info!("[cmd] check_custom_sounds");
    CustomSounds {
        start: false,
        stop: false,
    }
}
