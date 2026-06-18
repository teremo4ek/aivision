//! aivision — real Tauri command surface (logging stub).
//!
//! Every command the frontend invokes via `src/bindings.ts` is implemented in
//! the `commands` modules. Each logs its call to the console (`tauri-plugin-log`
//! -> Stdout + LogDir) and returns the same canned data the mock layer did, so
//! behavior is unchanged but the UI now talks to real Rust under `tauri dev`.

mod commands;
mod error;
mod events;
mod state;
mod types;

use std::sync::Mutex;

use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: None,
                    }),
                ])
                .build(),
        )
        .manage(Mutex::new(state::AppState::default()))
        .invoke_handler(tauri::generate_handler![
            // settings
            commands::settings::get_app_settings,
            commands::settings::get_default_settings,
            commands::settings::set_log_level,
            commands::settings::change_ptt_setting,
            commands::settings::change_audio_feedback_setting,
            commands::settings::change_audio_feedback_volume_setting,
            commands::settings::change_sound_theme_setting,
            commands::settings::change_start_hidden_setting,
            commands::settings::change_autostart_setting,
            commands::settings::change_translate_to_english_setting,
            commands::settings::change_selected_language_setting,
            commands::settings::change_overlay_position_setting,
            commands::settings::change_debug_mode_setting,
            commands::settings::change_word_correction_threshold_setting,
            commands::settings::change_extra_recording_buffer_setting,
            commands::settings::change_paste_delay_ms_setting,
            commands::settings::change_paste_method_setting,
            commands::settings::change_typing_tool_setting,
            commands::settings::change_external_script_path_setting,
            commands::settings::change_clipboard_handling_setting,
            commands::settings::change_auto_submit_setting,
            commands::settings::change_auto_submit_key_setting,
            commands::settings::change_post_process_enabled_setting,
            commands::settings::change_experimental_enabled_setting,
            commands::settings::change_mute_while_recording_setting,
            commands::settings::change_append_trailing_space_setting,
            commands::settings::change_lazy_stream_close_setting,
            commands::settings::change_app_language_setting,
            commands::settings::change_update_checks_setting,
            commands::settings::change_show_tray_icon_setting,
            commands::settings::change_whisper_accelerator_setting,
            commands::settings::change_ort_accelerator_setting,
            commands::settings::change_whisper_gpu_device,
            // shortcuts / keyboard
            commands::shortcuts::change_binding,
            commands::shortcuts::reset_binding,
            commands::shortcuts::suspend_binding,
            commands::shortcuts::resume_binding,
            commands::shortcuts::start_aivision_keys_recording,
            commands::shortcuts::stop_aivision_keys_recording,
            commands::shortcuts::initialize_shortcuts,
            commands::shortcuts::initialize_enigo,
            commands::shortcuts::get_keyboard_implementation,
            commands::shortcuts::get_available_typing_tools,
            commands::shortcuts::change_keyboard_implementation_setting,
            // models
            commands::models::get_available_models,
            commands::models::get_model_info,
            commands::models::download_model,
            commands::models::delete_model,
            commands::models::cancel_download,
            commands::models::set_active_model,
            commands::models::get_current_model,
            commands::models::get_transcription_model_status,
            commands::models::is_model_loading,
            commands::models::has_any_models_available,
            commands::models::has_any_models_or_downloads,
            commands::models::get_model_load_status,
            commands::models::unload_model_manually,
            commands::models::set_model_unload_timeout,
            // audio
            commands::audio::get_available_microphones,
            commands::audio::get_available_output_devices,
            commands::audio::get_microphone_mode,
            commands::audio::update_microphone_mode,
            commands::audio::get_selected_microphone,
            commands::audio::set_selected_microphone,
            commands::audio::get_selected_output_device,
            commands::audio::set_selected_output_device,
            commands::audio::get_clamshell_microphone,
            commands::audio::set_clamshell_microphone,
            commands::audio::play_test_sound,
            commands::audio::check_custom_sounds,
            // post-processing
            commands::post_process::change_post_process_base_url_setting,
            commands::post_process::change_post_process_api_key_setting,
            commands::post_process::change_post_process_model_setting,
            commands::post_process::set_post_process_provider,
            commands::post_process::fetch_post_process_models,
            commands::post_process::add_post_process_prompt,
            commands::post_process::update_post_process_prompt,
            commands::post_process::delete_post_process_prompt,
            commands::post_process::set_post_process_selected_prompt,
            commands::post_process::update_custom_words,
            // history
            commands::history::get_history_entries,
            commands::history::toggle_history_entry_saved,
            commands::history::get_audio_file_path,
            commands::history::delete_history_entry,
            commands::history::retry_history_entry_transcription,
            commands::history::update_history_limit,
            commands::history::update_recording_retention_period,
            // system / platform
            commands::system::get_app_dir_path,
            commands::system::get_log_dir_path,
            commands::system::open_recordings_folder,
            commands::system::open_log_dir,
            commands::system::open_app_data_dir,
            commands::system::is_portable,
            commands::system::is_laptop,
            commands::system::is_recording,
            commands::system::check_apple_intelligence_available,
            commands::system::get_available_accelerators,
            commands::system::cancel_operation,
            commands::system::trigger_update_check,
            commands::system::show_main_window_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
