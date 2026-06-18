//! History commands. `get_history_entries` returns the canned list (pagination
//! args accepted but ignored, like the mock); `toggle_history_entry_saved` and
//! `delete_history_entry` mutate the in-memory list so re-fetches stay
//! consistent; the rest are no-ops.

use std::sync::Mutex;

use tauri::State;

use crate::error::AppResult;
use crate::state::AppState;
use crate::types::PaginatedHistory;

#[tauri::command]
pub fn get_history_entries(
    state: State<'_, Mutex<AppState>>,
    cursor: Option<i64>,
    limit: Option<i64>,
) -> AppResult<PaginatedHistory> {
    log::info!("[cmd] get_history_entries cursor={cursor:?} limit={limit:?}");
    let entries = state.lock().unwrap().history.clone();
    Ok(PaginatedHistory {
        entries,
        has_more: false,
    })
}

#[tauri::command]
pub fn toggle_history_entry_saved(
    state: State<'_, Mutex<AppState>>,
    id: i64,
) -> AppResult<()> {
    log::info!("[cmd] toggle_history_entry_saved id={id}");
    let mut s = state.lock().unwrap();
    if let Some(entry) = s.history.iter_mut().find(|e| e.id == id) {
        entry.saved = !entry.saved;
    }
    Ok(())
}

#[tauri::command]
pub fn get_audio_file_path(file_name: String) -> AppResult<String> {
    log::info!("[cmd] get_audio_file_path file_name={file_name}");
    Ok(String::new())
}

#[tauri::command]
pub fn delete_history_entry(state: State<'_, Mutex<AppState>>, id: i64) -> AppResult<()> {
    log::info!("[cmd] delete_history_entry id={id}");
    state.lock().unwrap().history.retain(|e| e.id != id);
    Ok(())
}

#[tauri::command]
pub fn retry_history_entry_transcription(id: i64) -> AppResult<()> {
    log::info!("[cmd] retry_history_entry_transcription id={id}");
    Ok(())
}

#[tauri::command]
pub fn update_history_limit(state: State<'_, Mutex<AppState>>, limit: i64) -> AppResult<()> {
    log::info!("[cmd] update_history_limit limit={limit}");
    state.lock().unwrap().settings.history_limit = Some(limit);
    Ok(())
}

#[tauri::command]
pub fn update_recording_retention_period(period: String) -> AppResult<()> {
    log::info!("[cmd] update_recording_retention_period period={period}");
    Ok(())
}
