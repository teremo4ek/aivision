//! Post-processing (LLM provider/prompt) commands. Mutators are no-ops;
//! `fetch_post_process_models` returns an empty list (no keys/providers
//! configured in the stub); `add_post_process_prompt` returns a new prompt
//! with a generated id so the frontend can append it.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::AppResult;
use crate::types::LLMPrompt;

static PROMPT_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

#[tauri::command]
pub fn change_post_process_base_url_setting(
    provider_id: String,
    base_url: String,
) -> AppResult<()> {
    log::info!(
        "[cmd] change_post_process_base_url_setting provider_id={provider_id} base_url={base_url}"
    );
    Ok(())
}

#[tauri::command]
pub fn change_post_process_api_key_setting(
    provider_id: String,
    api_key: String,
) -> AppResult<()> {
    log::info!(
        "[cmd] change_post_process_api_key_setting provider_id={provider_id} api_key=<{} chars>",
        api_key.len()
    );
    Ok(())
}

#[tauri::command]
pub fn change_post_process_model_setting(
    provider_id: String,
    model: String,
) -> AppResult<()> {
    log::info!(
        "[cmd] change_post_process_model_setting provider_id={provider_id} model={model}"
    );
    Ok(())
}

#[tauri::command]
pub fn set_post_process_provider(provider_id: String) -> AppResult<()> {
    log::info!("[cmd] set_post_process_provider provider_id={provider_id}");
    Ok(())
}

#[tauri::command]
pub fn fetch_post_process_models(_provider_id: String) -> AppResult<Vec<String>> {
    log::info!("[cmd] fetch_post_process_models provider_id={_provider_id}");
    Ok(Vec::new())
}

#[tauri::command]
pub fn add_post_process_prompt(name: String, prompt: String) -> AppResult<LLMPrompt> {
    log::info!("[cmd] add_post_process_prompt name={name}");
    let id = format!("prompt-{}", PROMPT_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
    Ok(LLMPrompt { id, name, prompt })
}

#[tauri::command]
pub fn update_post_process_prompt(id: String, name: String, prompt: String) -> AppResult<()> {
    log::info!(
        "[cmd] update_post_process_prompt id={id} name={name} prompt=<{} chars>",
        prompt.len()
    );
    Ok(())
}

#[tauri::command]
pub fn delete_post_process_prompt(id: String) -> AppResult<()> {
    log::info!("[cmd] delete_post_process_prompt id={id}");
    Ok(())
}

#[tauri::command]
pub fn set_post_process_selected_prompt(id: String) -> AppResult<()> {
    log::info!("[cmd] set_post_process_selected_prompt id={id}");
    Ok(())
}

#[tauri::command]
pub fn update_custom_words(words: Vec<String>) -> AppResult<()> {
    log::info!("[cmd] update_custom_words count={}", words.len());
    Ok(())
}
