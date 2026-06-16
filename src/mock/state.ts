/**
 * Mutable mock data backing the mock command layer (src/mock/core.ts).
 *
 * This stands in for the Rust backend's state. The UI treats it as read-only
 * data it would normally fetch via Tauri commands; only the simulated "model
 * download" flow mutates it (via markModelDownloaded / setCurrentModel).
 */
import type {
  AppSettings,
  AudioDevice,
  HistoryEntry,
  ModelInfo,
} from "@/bindings";

function makeSettings(): AppSettings {
  return {
    bindings: {
      transcribe: {
        id: "transcribe",
        name: "Transcribe",
        description: "Start recording and transcribe audio",
        default_binding: "cmd+shift+space",
        current_binding: "cmd+shift+space",
      },
      cancel: {
        id: "cancel",
        name: "Cancel",
        description: "Cancel the current recording",
        default_binding: "escape",
        current_binding: "escape",
      },
    },
    push_to_talk: false,
    audio_feedback: true,
    audio_feedback_volume: 50,
    sound_theme: "marimba",
    start_hidden: false,
    autostart_enabled: false,
    update_checks_enabled: true,
    selected_model: undefined,
    always_on_microphone: false,
    selected_microphone: "Default",
    clamshell_microphone: "Default",
    selected_output_device: "Default",
    translate_to_english: false,
    selected_language: "auto",
    overlay_position: "bottom",
    debug_mode: false,
    log_level: "info",
    custom_words: [],
    model_unload_timeout: "min_5",
    word_correction_threshold: 60,
    history_limit: 100,
    recording_retention_period: "weeks_2",
    paste_method: "ctrl_v",
    clipboard_handling: "dont_modify",
    auto_submit: false,
    auto_submit_key: "enter",
    post_process_enabled: false,
    post_process_provider_id: "openai",
    post_process_providers: [
      {
        id: "openai",
        label: "OpenAI",
        base_url: "https://api.openai.com/v1",
        models_endpoint: "/models",
        supports_structured_output: true,
      },
      {
        id: "groq",
        label: "Groq",
        base_url: "https://api.groq.com/openai/v1",
        models_endpoint: "/models",
      },
      {
        id: "custom",
        label: "Custom",
        base_url: "",
        allow_base_url_edit: true,
      },
    ],
    post_process_api_keys: {},
    post_process_models: {},
    post_process_prompts: [],
    post_process_selected_prompt_id: null,
    mute_while_recording: false,
    append_trailing_space: true,
    app_language: "en",
    experimental_enabled: false,
    lazy_stream_close: false,
    keyboard_implementation: "tauri",
    show_tray_icon: true,
    paste_delay_ms: 0,
    typing_tool: "auto",
    external_script_path: null,
    custom_filler_words: null,
    whisper_accelerator: "auto",
    ort_accelerator: "auto",
    whisper_gpu_device: 0,
    extra_recording_buffer_ms: 300,
  };
}

const models: ModelInfo[] = [
  {
    id: "small",
    name: "Whisper Small",
    description: "Fast and fairly accurate.",
    filename: "small.bin",
    url: "https://example.com/small.bin",
    sha256: null,
    size_mb: 488,
    is_downloaded: false,
    is_downloading: false,
    partial_size: 0,
    is_directory: false,
    engine_type: "Whisper",
    accuracy_score: 6,
    speed_score: 9,
    supports_translation: true,
    is_recommended: true,
    supported_languages: ["auto", "en", "ru"],
    supports_language_selection: true,
    is_custom: false,
  },
  {
    id: "turbo",
    name: "Whisper Turbo",
    description: "Balanced accuracy and speed.",
    filename: "turbo.bin",
    url: "https://example.com/turbo.bin",
    sha256: null,
    size_mb: 812,
    is_downloaded: false,
    is_downloading: false,
    partial_size: 0,
    is_directory: false,
    engine_type: "Whisper",
    accuracy_score: 8,
    speed_score: 7,
    supports_translation: true,
    is_recommended: false,
    supported_languages: ["auto", "en", "ru"],
    supports_language_selection: true,
    is_custom: false,
  },
  {
    id: "medium",
    name: "Whisper Medium",
    description: "Good accuracy, medium speed.",
    filename: "medium.bin",
    url: "https://example.com/medium.bin",
    sha256: null,
    size_mb: 1500,
    is_downloaded: false,
    is_downloading: false,
    partial_size: 0,
    is_directory: false,
    engine_type: "Whisper",
    accuracy_score: 8,
    speed_score: 5,
    supports_translation: true,
    is_recommended: false,
    supported_languages: ["auto", "en", "ru"],
    supports_language_selection: true,
    is_custom: false,
  },
];

const microphones: AudioDevice[] = [
  { index: "default", name: "Default", is_default: true },
  { index: "0", name: "MacBook Pro Microphone", is_default: false },
  { index: "1", name: "External Microphone", is_default: false },
];

const outputDevices: AudioDevice[] = [
  { index: "default", name: "Default", is_default: true },
  { index: "1", name: "Built-in Output", is_default: false },
];

const history: HistoryEntry[] = [
  {
    id: 1,
    file_name: "recording-1.wav",
    timestamp: 1718500000000,
    saved: true,
    title: "Meeting notes",
    transcription_text:
      "This is a mock transcription entry demonstrating the history view.",
    post_processed_text: null,
    post_process_prompt: null,
    post_process_requested: false,
  },
  {
    id: 2,
    file_name: "recording-2.wav",
    timestamp: 1718400000000,
    saved: false,
    title: "Voice memo",
    transcription_text:
      "Another mock entry so the history list is not empty in the UI demo.",
    post_processed_text: null,
    post_process_prompt: null,
    post_process_requested: false,
  },
];

export const state = {
  settings: makeSettings(),
  defaultSettings: makeSettings(),
  models,
  microphones,
  outputDevices,
  history,
  currentModel: "",
  downloaded: new Set<string>(),
};

export function markModelDownloaded(modelId: string): void {
  state.downloaded.add(modelId);
}

export function removeModel(modelId: string): void {
  state.downloaded.delete(modelId);
  if (state.currentModel === modelId) state.currentModel = "";
}

export function setCurrentModel(modelId: string): void {
  state.currentModel = modelId;
  state.settings.selected_model = modelId;
}
