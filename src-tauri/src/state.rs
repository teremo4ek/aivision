//! In-process application state, mirroring `src/mock/state.ts`.
//!
//! This is the backing store for the logging stub. Getters read from here,
//! mutators no-op (or lightly mutate, as the mock does), and the simulated
//! model download flips entries in `downloaded`.

use std::collections::HashSet;

use crate::types::{
    AppSettings, AudioDevice, EngineType, HistoryEntry, ModelInfo, ShortcutBinding,
};

/// Raw model catalogue without the runtime `is_downloaded` flag, which is
/// stamped in from `downloaded` at read time (matching `mock/core.ts`).
#[derive(Debug, Clone)]
struct ModelRecord {
    id: String,
    name: String,
    description: String,
    filename: String,
    url: Option<String>,
    sha256: Option<String>,
    size_mb: i64,
    engine_type: EngineType,
    accuracy_score: i64,
    speed_score: i64,
    supports_translation: bool,
    is_recommended: bool,
    supported_languages: Vec<String>,
    supports_language_selection: bool,
}

impl ModelRecord {
    fn to_info(&self, is_downloaded: bool) -> ModelInfo {
        ModelInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            filename: self.filename.clone(),
            url: self.url.clone(),
            sha256: self.sha256.clone(),
            size_mb: self.size_mb,
            is_downloaded,
            is_downloading: false,
            partial_size: 0,
            is_directory: false,
            engine_type: self.engine_type,
            accuracy_score: self.accuracy_score,
            speed_score: self.speed_score,
            supports_translation: self.supports_translation,
            is_recommended: self.is_recommended,
            supported_languages: self.supported_languages.clone(),
            supports_language_selection: self.supports_language_selection,
            is_custom: false,
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub settings: AppSettings,
    pub default_settings: AppSettings,
    models: Vec<ModelRecord>,
    pub microphones: Vec<AudioDevice>,
    pub output_devices: Vec<AudioDevice>,
    pub history: Vec<HistoryEntry>,
    pub current_model: String,
    pub downloaded: HashSet<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: AppSettings::mock(),
            default_settings: AppSettings::mock(),
            models: mock_models(),
            microphones: mock_microphones(),
            output_devices: mock_outputs(),
            history: mock_history(),
            current_model: String::new(),
            downloaded: HashSet::new(),
        }
    }
}

impl AppState {
    /// Models with `is_downloaded` stamped from the `downloaded` set.
    pub fn models(&self) -> Vec<ModelInfo> {
        self.models
            .iter()
            .map(|m| m.to_info(self.downloaded.contains(&m.id)))
            .collect()
    }

    /// Look up a single model's info, or `None` if unknown.
    pub fn model_info(&self, model_id: &str) -> Option<ModelInfo> {
        self.models
            .iter()
            .find(|m| m.id == model_id)
            .map(|m| m.to_info(self.downloaded.contains(&m.id)))
    }

    pub fn model_size_mb(&self, model_id: &str) -> i64 {
        self.models
            .iter()
            .find(|m| m.id == model_id)
            .map(|m| m.size_mb)
            .unwrap_or(100)
    }

    pub fn mark_downloaded(&mut self, model_id: &str) {
        self.downloaded.insert(model_id.to_string());
    }

    pub fn remove_model(&mut self, model_id: &str) {
        self.downloaded.remove(model_id);
        if self.current_model == model_id {
            self.current_model.clear();
        }
    }

    pub fn set_current_model(&mut self, model_id: &str) {
        self.current_model = model_id.to_string();
        self.settings.selected_model = Some(model_id.to_string());
    }

    /// Update a shortcut binding's current binding; returns the stored binding.
    pub fn set_binding(&mut self, id: &str, binding: &str) -> Option<ShortcutBinding> {
        if let Some(entry) = self.settings.bindings.get_mut(id) {
            entry.current_binding = binding.to_string();
            Some(entry.clone())
        } else {
            None
        }
    }

    pub fn reset_binding(&mut self, id: &str) -> Option<ShortcutBinding> {
        if let Some(entry) = self.settings.bindings.get_mut(id) {
            entry.current_binding = entry.default_binding.clone();
            Some(entry.clone())
        } else {
            None
        }
    }
}

fn mock_models() -> Vec<ModelRecord> {
    let langs = vec!["auto".to_string(), "en".to_string(), "ru".to_string()];
    vec![
        ModelRecord {
            id: "small".to_string(),
            name: "Whisper Small".to_string(),
            description: "Fast and fairly accurate.".to_string(),
            filename: "small.bin".to_string(),
            url: Some("https://example.com/small.bin".to_string()),
            sha256: None,
            size_mb: 488,
            engine_type: EngineType::Whisper,
            accuracy_score: 6,
            speed_score: 9,
            supports_translation: true,
            is_recommended: true,
            supported_languages: langs.clone(),
            supports_language_selection: true,
        },
        ModelRecord {
            id: "turbo".to_string(),
            name: "Whisper Turbo".to_string(),
            description: "Balanced accuracy and speed.".to_string(),
            filename: "turbo.bin".to_string(),
            url: Some("https://example.com/turbo.bin".to_string()),
            sha256: None,
            size_mb: 812,
            engine_type: EngineType::Whisper,
            accuracy_score: 8,
            speed_score: 7,
            supports_translation: true,
            is_recommended: false,
            supported_languages: langs.clone(),
            supports_language_selection: true,
        },
        ModelRecord {
            id: "medium".to_string(),
            name: "Whisper Medium".to_string(),
            description: "Good accuracy, medium speed.".to_string(),
            filename: "medium.bin".to_string(),
            url: Some("https://example.com/medium.bin".to_string()),
            sha256: None,
            size_mb: 1500,
            engine_type: EngineType::Whisper,
            accuracy_score: 8,
            speed_score: 5,
            supports_translation: true,
            is_recommended: false,
            supported_languages: langs,
            supports_language_selection: true,
        },
    ]
}

fn mock_microphones() -> Vec<AudioDevice> {
    vec![
        AudioDevice {
            index: "default".to_string(),
            name: "Default".to_string(),
            is_default: true,
        },
        AudioDevice {
            index: "0".to_string(),
            name: "MacBook Pro Microphone".to_string(),
            is_default: false,
        },
        AudioDevice {
            index: "1".to_string(),
            name: "External Microphone".to_string(),
            is_default: false,
        },
    ]
}

fn mock_outputs() -> Vec<AudioDevice> {
    vec![
        AudioDevice {
            index: "default".to_string(),
            name: "Default".to_string(),
            is_default: true,
        },
        AudioDevice {
            index: "1".to_string(),
            name: "Built-in Output".to_string(),
            is_default: false,
        },
    ]
}

fn mock_history() -> Vec<HistoryEntry> {
    vec![
        HistoryEntry {
            id: 1,
            file_name: "recording-1.wav".to_string(),
            timestamp: 1_718_500_000_000,
            saved: true,
            title: "Meeting notes".to_string(),
            transcription_text:
                "This is a mock transcription entry demonstrating the history view.".to_string(),
            post_processed_text: None,
            post_process_prompt: None,
            post_process_requested: false,
        },
        HistoryEntry {
            id: 2,
            file_name: "recording-2.wav".to_string(),
            timestamp: 1_718_400_000_000,
            saved: false,
            title: "Voice memo".to_string(),
            transcription_text:
                "Another mock entry so the history list is not empty in the UI demo.".to_string(),
            post_processed_text: None,
            post_process_prompt: None,
            post_process_requested: false,
        },
    ]
}
