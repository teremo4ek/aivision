//! Transcription model commands. `download_model` kicks off the simulated
//! download (see `events::simulate_download`); the rest read from or lightly
//! mutate `AppState`, mirroring `src/mock/core.ts`.

use std::sync::Mutex;

use tauri::{AppHandle, State};

use crate::error::AppResult;
use crate::events::simulate_download;
use crate::state::AppState;
use crate::types::{ModelInfo, ModelLoadStatus};

#[tauri::command]
pub fn get_available_models(
    state: State<'_, Mutex<AppState>>,
) -> AppResult<Vec<ModelInfo>> {
    log::info!("[cmd] get_available_models");
    Ok(state.lock().unwrap().models())
}

#[tauri::command]
pub fn get_model_info(
    state: State<'_, Mutex<AppState>>,
    model_id: String,
) -> AppResult<Option<ModelInfo>> {
    log::info!("[cmd] get_model_info model_id={model_id}");
    Ok(state.lock().unwrap().model_info(&model_id))
}

#[tauri::command]
pub fn download_model(app: AppHandle, model_id: String) -> AppResult<()> {
    log::info!("[cmd] download_model model_id={model_id}");
    simulate_download(app, model_id);
    Ok(())
}

#[tauri::command]
pub fn delete_model(state: State<'_, Mutex<AppState>>, model_id: String) -> AppResult<()> {
    log::info!("[cmd] delete_model model_id={model_id}");
    state.lock().unwrap().remove_model(&model_id);
    Ok(())
}

#[tauri::command]
pub fn cancel_download(model_id: String) -> AppResult<()> {
    log::info!("[cmd] cancel_download model_id={model_id}");
    Ok(())
}

#[tauri::command]
pub fn set_active_model(state: State<'_, Mutex<AppState>>, model_id: String) -> AppResult<()> {
    log::info!("[cmd] set_active_model model_id={model_id}");
    state.lock().unwrap().set_current_model(&model_id);
    Ok(())
}

#[tauri::command]
pub fn get_current_model(state: State<'_, Mutex<AppState>>) -> AppResult<String> {
    log::info!("[cmd] get_current_model");
    Ok(state.lock().unwrap().current_model.clone())
}

#[tauri::command]
pub fn get_transcription_model_status() -> AppResult<Option<String>> {
    log::info!("[cmd] get_transcription_model_status");
    Ok(None)
}

#[tauri::command]
pub fn is_model_loading() -> AppResult<bool> {
    log::info!("[cmd] is_model_loading");
    Ok(false)
}

#[tauri::command]
pub fn has_any_models_available(state: State<'_, Mutex<AppState>>) -> AppResult<bool> {
    log::info!("[cmd] has_any_models_available");
    Ok(!state.lock().unwrap().downloaded.is_empty())
}

#[tauri::command]
pub fn has_any_models_or_downloads(state: State<'_, Mutex<AppState>>) -> AppResult<bool> {
    log::info!("[cmd] has_any_models_or_downloads");
    Ok(!state.lock().unwrap().downloaded.is_empty())
}

#[tauri::command]
pub fn get_model_load_status(state: State<'_, Mutex<AppState>>) -> AppResult<ModelLoadStatus> {
    log::info!("[cmd] get_model_load_status");
    let s = state.lock().unwrap();
    Ok(ModelLoadStatus {
        is_loaded: !s.current_model.is_empty(),
        current_model: if s.current_model.is_empty() {
            None
        } else {
            Some(s.current_model.clone())
        },
    })
}

#[tauri::command]
pub fn unload_model_manually() -> AppResult<()> {
    log::info!("[cmd] unload_model_manually");
    Ok(())
}

/// Bare return (`Promise<void>`).
#[tauri::command]
pub fn set_model_unload_timeout(timeout: String) {
    log::info!("[cmd] set_model_unload_timeout timeout={timeout}");
}
