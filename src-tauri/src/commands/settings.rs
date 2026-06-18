//! Settings commands: getters (`get_app_settings`, `get_default_settings`),
//! `set_log_level`, and the `change_*_setting` mutators. Mirrors the mock —
//! mutators are accepted and logged but otherwise no-ops (the Zustand stores
//! already apply changes optimistically).

use std::sync::Mutex;

use tauri::State;

use crate::error::AppResult;
use crate::state::AppState;
use crate::types::AppSettings;

#[tauri::command]
pub fn get_app_settings(state: State<'_, Mutex<AppState>>) -> AppResult<AppSettings> {
    log::info!("[cmd] get_app_settings");
    Ok(state.lock().unwrap().settings.clone())
}

#[tauri::command]
pub fn get_default_settings(state: State<'_, Mutex<AppState>>) -> AppResult<AppSettings> {
    log::info!("[cmd] get_default_settings");
    Ok(state.lock().unwrap().default_settings.clone())
}

#[tauri::command]
pub fn set_log_level(level: String) -> AppResult<()> {
    log::info!("[cmd] set_log_level level={level}");
    let filter = match level.as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    };
    log::set_max_level(filter);
    Ok(())
}

// --- change_* mutators (no-ops, mirroring the mock) -----------------------

#[tauri::command]
pub fn change_ptt_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_ptt_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_audio_feedback_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_audio_feedback_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_audio_feedback_volume_setting(volume: i64) -> AppResult<()> {
    log::info!("[cmd] change_audio_feedback_volume_setting volume={volume}");
    Ok(())
}

#[tauri::command]
pub fn change_sound_theme_setting(theme: String) -> AppResult<()> {
    log::info!("[cmd] change_sound_theme_setting theme={theme}");
    Ok(())
}

#[tauri::command]
pub fn change_start_hidden_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_start_hidden_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_autostart_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_autostart_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_translate_to_english_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_translate_to_english_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_selected_language_setting(language: String) -> AppResult<()> {
    log::info!("[cmd] change_selected_language_setting language={language}");
    Ok(())
}

#[tauri::command]
pub fn change_overlay_position_setting(position: String) -> AppResult<()> {
    log::info!("[cmd] change_overlay_position_setting position={position}");
    Ok(())
}

#[tauri::command]
pub fn change_debug_mode_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_debug_mode_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_word_correction_threshold_setting(threshold: i64) -> AppResult<()> {
    log::info!("[cmd] change_word_correction_threshold_setting threshold={threshold}");
    Ok(())
}

#[tauri::command]
pub fn change_extra_recording_buffer_setting(ms: i64) -> AppResult<()> {
    log::info!("[cmd] change_extra_recording_buffer_setting ms={ms}");
    Ok(())
}

#[tauri::command]
pub fn change_paste_delay_ms_setting(ms: i64) -> AppResult<()> {
    log::info!("[cmd] change_paste_delay_ms_setting ms={ms}");
    Ok(())
}

#[tauri::command]
pub fn change_paste_method_setting(method: String) -> AppResult<()> {
    log::info!("[cmd] change_paste_method_setting method={method}");
    Ok(())
}

#[tauri::command]
pub fn change_typing_tool_setting(tool: String) -> AppResult<()> {
    log::info!("[cmd] change_typing_tool_setting tool={tool}");
    Ok(())
}

#[tauri::command]
pub fn change_external_script_path_setting(path: Option<String>) -> AppResult<()> {
    log::info!("[cmd] change_external_script_path_setting path={path:?}");
    Ok(())
}

#[tauri::command]
pub fn change_clipboard_handling_setting(handling: String) -> AppResult<()> {
    log::info!("[cmd] change_clipboard_handling_setting handling={handling}");
    Ok(())
}

#[tauri::command]
pub fn change_auto_submit_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_auto_submit_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_auto_submit_key_setting(key: String) -> AppResult<()> {
    log::info!("[cmd] change_auto_submit_key_setting key={key}");
    Ok(())
}

#[tauri::command]
pub fn change_post_process_enabled_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_post_process_enabled_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_experimental_enabled_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_experimental_enabled_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_mute_while_recording_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_mute_while_recording_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_append_trailing_space_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_append_trailing_space_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_lazy_stream_close_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_lazy_stream_close_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_app_language_setting(language: String) -> AppResult<()> {
    log::info!("[cmd] change_app_language_setting language={language}");
    Ok(())
}

#[tauri::command]
pub fn change_update_checks_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_update_checks_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_show_tray_icon_setting(enabled: bool) -> AppResult<()> {
    log::info!("[cmd] change_show_tray_icon_setting enabled={enabled}");
    Ok(())
}

#[tauri::command]
pub fn change_whisper_accelerator_setting(accelerator: String) -> AppResult<()> {
    log::info!("[cmd] change_whisper_accelerator_setting accelerator={accelerator}");
    Ok(())
}

#[tauri::command]
pub fn change_ort_accelerator_setting(accelerator: String) -> AppResult<()> {
    log::info!("[cmd] change_ort_accelerator_setting accelerator={accelerator}");
    Ok(())
}

#[tauri::command]
pub fn change_whisper_gpu_device(device: i64) -> AppResult<()> {
    log::info!("[cmd] change_whisper_gpu_device device={device}");
    Ok(())
}
