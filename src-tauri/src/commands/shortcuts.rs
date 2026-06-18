//! Keyboard shortcut / implementation commands. `change_binding` and
//! `reset_binding` return a real `BindingResponse` (the mock returned null,
//! which would have thrown in the store); the rest are no-ops.

use std::sync::Mutex;

use tauri::State;

use crate::error::AppResult;
use crate::state::AppState;
use crate::types::{BindingResponse, ImplementationChangeResult, KeyboardImplementation};

#[tauri::command]
pub fn change_binding(
    state: State<'_, Mutex<AppState>>,
    id: String,
    binding: String,
) -> AppResult<BindingResponse> {
    log::info!("[cmd] change_binding id={id} binding={binding}");
    let binding_record = {
        let mut s = state.lock().unwrap();
        s.set_binding(&id, &binding)
    };
    Ok(BindingResponse {
        success: true,
        binding: binding_record,
        error: None,
    })
}

#[tauri::command]
pub fn reset_binding(
    state: State<'_, Mutex<AppState>>,
    id: String,
) -> AppResult<BindingResponse> {
    log::info!("[cmd] reset_binding id={id}");
    let binding_record = {
        let mut s = state.lock().unwrap();
        s.reset_binding(&id)
    };
    Ok(BindingResponse {
        success: true,
        binding: binding_record,
        error: None,
    })
}

#[tauri::command]
pub fn suspend_binding(id: String) -> AppResult<()> {
    log::info!("[cmd] suspend_binding id={id}");
    Ok(())
}

#[tauri::command]
pub fn resume_binding(id: String) -> AppResult<()> {
    log::info!("[cmd] resume_binding id={id}");
    Ok(())
}

#[tauri::command]
pub fn start_aivision_keys_recording(binding_id: String) -> AppResult<()> {
    log::info!("[cmd] start_aivision_keys_recording binding_id={binding_id}");
    Ok(())
}

#[tauri::command]
pub fn stop_aivision_keys_recording() -> AppResult<()> {
    log::info!("[cmd] stop_aivision_keys_recording");
    Ok(())
}

#[tauri::command]
pub fn initialize_shortcuts() -> AppResult<()> {
    log::info!("[cmd] initialize_shortcuts");
    Ok(())
}

#[tauri::command]
pub fn initialize_enigo() -> AppResult<()> {
    log::info!("[cmd] initialize_enigo");
    Ok(())
}

/// Bare return (`Promise<string>`): the current keyboard implementation.
#[tauri::command]
pub fn get_keyboard_implementation(state: State<'_, Mutex<AppState>>) -> String {
    log::info!("[cmd] get_keyboard_implementation");
    match state.lock().unwrap().settings.keyboard_implementation {
        Some(KeyboardImplementation::AivisionKeys) => "aivision_keys".to_string(),
        _ => "tauri".to_string(),
    }
}

/// Bare return (`Promise<string[]>`).
#[tauri::command]
pub fn get_available_typing_tools() -> Vec<String> {
    log::info!("[cmd] get_available_typing_tools");
    vec![
        "auto".to_string(),
        "wtype".to_string(),
        "kwtype".to_string(),
        "dotool".to_string(),
        "ydotool".to_string(),
        "xdotool".to_string(),
    ]
}

#[tauri::command]
pub fn change_keyboard_implementation_setting(
    implementation: String,
) -> AppResult<ImplementationChangeResult> {
    log::info!("[cmd] change_keyboard_implementation_setting implementation={implementation}");
    Ok(ImplementationChangeResult {
        success: true,
        reset_bindings: Vec::new(),
    })
}
