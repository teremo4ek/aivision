//! Serde types mirroring the TypeScript contract in `src/bindings.ts`.
//!
//! These are the wire shapes the frontend expects. Field names are snake_case
//! (matching the TS types verbatim, e.g. `is_downloaded`), so no `rename_all`
//! is needed on structs. String-union enums use `rename_all = "snake_case"`
//! except `EngineType`, whose variants are PascalCase on the wire (`"Whisper"`).
//!
//! Optional-vs-nullable distinction (read from the TS `?`):
//! - `field?: T`         -> `Option<T>` with `skip_serializing_if` (omitted when `None`)
//! - `field: T | null`   -> `Option<T>` WITHOUT skip (serialized as `null` when `None`)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// String-union enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SoundTheme {
    Marimba,
    Pop,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayPosition {
    None,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelUnloadTimeout {
    Never,
    Immediately,
    Min2,
    Min5,
    Min10,
    Min15,
    Hour1,
    Sec15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingRetentionPeriod {
    Never,
    PreserveLimit,
    Days3,
    Weeks2,
    Months3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasteMethod {
    CtrlV,
    Direct,
    None,
    ShiftInsert,
    CtrlShiftV,
    ExternalScript,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardHandling {
    DontModify,
    CopyToClipboard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoSubmitKey {
    Enter,
    CtrlEnter,
    CmdEnter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardImplementation {
    Tauri,
    AivisionKeys,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypingTool {
    Auto,
    Wtype,
    Kwtype,
    Dotool,
    Ydotool,
    Xdotool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WhisperAcceleratorSetting {
    Auto,
    Cpu,
    Gpu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrtAcceleratorSetting {
    Auto,
    Cpu,
    Cuda,
    Directml,
    Rocm,
}

/// Engine variants are PascalCase on the wire (`"Whisper"`, `"MoonshineStreaming"`),
/// so this enum intentionally does NOT use `rename_all`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineType {
    Whisper,
    Parakeet,
    Moonshine,
    MoonshineStreaming,
    SenseVoice,
    GigaAM,
    Canary,
    Cohere,
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutBinding {
    pub id: String,
    pub name: String,
    pub description: String,
    pub default_binding: String,
    pub current_binding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProcessProvider {
    pub id: String,
    pub label: String,
    pub base_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_base_url_edit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supports_structured_output: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMPrompt {
    pub id: String,
    pub name: String,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub filename: String,
    pub url: Option<String>,
    pub sha256: Option<String>,
    pub size_mb: i64,
    pub is_downloaded: bool,
    pub is_downloading: bool,
    pub partial_size: i64,
    pub is_directory: bool,
    pub engine_type: EngineType,
    pub accuracy_score: i64,
    pub speed_score: i64,
    pub supports_translation: bool,
    pub is_recommended: bool,
    pub supported_languages: Vec<String>,
    pub supports_language_selection: bool,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLoadStatus {
    pub is_loaded: bool,
    /// `string | null` (no `?`) -> emitted as `null` when empty.
    pub current_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub index: String,
    pub name: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDeviceOption {
    pub id: i64,
    pub name: String,
    pub total_vram_mb: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableAccelerators {
    pub whisper: Vec<String>,
    pub ort: Vec<String>,
    pub gpu_devices: Vec<GpuDeviceOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSounds {
    pub start: bool,
    pub stop: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub file_name: String,
    pub timestamp: i64,
    pub saved: bool,
    pub title: String,
    pub transcription_text: String,
    /// `string | null` (no `?`) -> emitted as `null` when empty.
    pub post_processed_text: Option<String>,
    /// `string | null` (no `?`) -> emitted as `null` when empty.
    pub post_process_prompt: Option<String>,
    pub post_process_requested: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedHistory {
    pub entries: Vec<HistoryEntry>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingResponse {
    pub success: bool,
    /// `ShortcutBinding | null` (no `?`) -> emitted as `null` when empty.
    pub binding: Option<ShortcutBinding>,
    /// `string | null` (no `?`) -> emitted as `null` when empty.
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationChangeResult {
    pub success: bool,
    pub reset_bindings: Vec<String>,
}

/// Mirrors `AppSettings` in `bindings.ts`. Most fields are optional (`?` ->
/// omitted when `None`); `external_script_path` is `string | null` (always
/// present, `null` when unset).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub bindings: HashMap<String, ShortcutBinding>,
    pub push_to_talk: bool,
    pub audio_feedback: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_feedback_volume: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_theme: Option<SoundTheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autostart_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_checks_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always_on_microphone: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_microphone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clamshell_microphone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_output_device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translate_to_english: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlay_position: Option<OverlayPosition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<LogLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_unload_timeout: Option<ModelUnloadTimeout>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_correction_threshold: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recording_retention_period: Option<RecordingRetentionPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paste_method: Option<PasteMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clipboard_handling: Option<ClipboardHandling>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_submit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_submit_key: Option<AutoSubmitKey>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_process_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_process_provider_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_process_providers: Option<Vec<PostProcessProvider>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_process_api_keys: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_process_models: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_process_prompts: Option<Vec<LLMPrompt>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_process_selected_prompt_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute_while_recording: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub append_trailing_space: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lazy_stream_close: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_implementation: Option<KeyboardImplementation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_tray_icon: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paste_delay_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub typing_tool: Option<TypingTool>,
    /// `string | null` (required, no `?`) -> serialized as `null` when unset.
    pub external_script_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_filler_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whisper_accelerator: Option<WhisperAcceleratorSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ort_accelerator: Option<OrtAcceleratorSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whisper_gpu_device: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_recording_buffer_ms: Option<i64>,
}

impl AppSettings {
    /// Build the canned settings object, mirroring `src/mock/state.ts::makeSettings()`.
    pub fn mock() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(
            "transcribe".to_string(),
            ShortcutBinding {
                id: "transcribe".to_string(),
                name: "Transcribe".to_string(),
                description: "Start recording and transcribe audio".to_string(),
                default_binding: "cmd+shift+space".to_string(),
                current_binding: "cmd+shift+space".to_string(),
            },
        );
        bindings.insert(
            "cancel".to_string(),
            ShortcutBinding {
                id: "cancel".to_string(),
                name: "Cancel".to_string(),
                description: "Cancel the current recording".to_string(),
                default_binding: "escape".to_string(),
                current_binding: "escape".to_string(),
            },
        );

        Self {
            bindings,
            push_to_talk: false,
            audio_feedback: true,
            audio_feedback_volume: Some(50),
            sound_theme: Some(SoundTheme::Marimba),
            start_hidden: Some(false),
            autostart_enabled: Some(false),
            update_checks_enabled: Some(true),
            selected_model: None,
            always_on_microphone: Some(false),
            selected_microphone: Some("Default".to_string()),
            clamshell_microphone: Some("Default".to_string()),
            selected_output_device: Some("Default".to_string()),
            translate_to_english: Some(false),
            selected_language: Some("auto".to_string()),
            overlay_position: Some(OverlayPosition::Bottom),
            debug_mode: Some(false),
            log_level: Some(LogLevel::Info),
            custom_words: Some(Vec::new()),
            model_unload_timeout: Some(ModelUnloadTimeout::Min5),
            word_correction_threshold: Some(60),
            history_limit: Some(100),
            recording_retention_period: Some(RecordingRetentionPeriod::Weeks2),
            paste_method: Some(PasteMethod::CtrlV),
            clipboard_handling: Some(ClipboardHandling::DontModify),
            auto_submit: Some(false),
            auto_submit_key: Some(AutoSubmitKey::Enter),
            post_process_enabled: Some(false),
            post_process_provider_id: Some("openai".to_string()),
            post_process_providers: Some(vec![
                PostProcessProvider {
                    id: "openai".to_string(),
                    label: "OpenAI".to_string(),
                    base_url: "https://api.openai.com/v1".to_string(),
                    allow_base_url_edit: None,
                    models_endpoint: Some("/models".to_string()),
                    supports_structured_output: Some(true),
                },
                PostProcessProvider {
                    id: "groq".to_string(),
                    label: "Groq".to_string(),
                    base_url: "https://api.groq.com/openai/v1".to_string(),
                    allow_base_url_edit: None,
                    models_endpoint: Some("/models".to_string()),
                    supports_structured_output: None,
                },
                PostProcessProvider {
                    id: "custom".to_string(),
                    label: "Custom".to_string(),
                    base_url: String::new(),
                    allow_base_url_edit: Some(true),
                    models_endpoint: None,
                    supports_structured_output: None,
                },
            ]),
            post_process_api_keys: Some(HashMap::new()),
            post_process_models: Some(HashMap::new()),
            post_process_prompts: Some(Vec::new()),
            post_process_selected_prompt_id: None,
            mute_while_recording: Some(false),
            append_trailing_space: Some(true),
            app_language: Some("en".to_string()),
            experimental_enabled: Some(false),
            lazy_stream_close: Some(false),
            keyboard_implementation: Some(KeyboardImplementation::Tauri),
            show_tray_icon: Some(true),
            paste_delay_ms: Some(0),
            typing_tool: Some(TypingTool::Auto),
            external_script_path: None,
            custom_filler_words: None,
            whisper_accelerator: Some(WhisperAcceleratorSetting::Auto),
            ort_accelerator: Some(OrtAcceleratorSetting::Auto),
            whisper_gpu_device: Some(0),
            extra_recording_buffer_ms: Some(300),
        }
    }
}
