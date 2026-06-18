//! Backend-to-frontend events for the simulated model-download flow.
//!
//! Mirrors `simulateDownload` in `src/mock/core.ts`: emit `model-download-progress`
//! over ~12 steps, then the verification/extraction/complete events. The frontend
//! (`stores/modelStore.ts`) listens for these exact kebab-case event names.

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

use crate::state::AppState;

/// Payload for `model-download-progress`. Field names are snake_case to match
/// the mock and the frontend's `DownloadProgress` type.
#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub model_id: String,
    pub downloaded: f64,
    pub total: f64,
    pub percentage: i64,
}

/// Run the simulated download on a background thread, emitting events exactly
/// like the mock. Returns immediately; the AppHandle is `Clone + Send`.
pub fn simulate_download(app: AppHandle, model_id: String) {
    let size_mb = app
        .state::<Mutex<AppState>>()
        .lock()
        .map(|s| s.model_size_mb(&model_id))
        .unwrap_or(100);
    let total = (size_mb.max(1) as f64) * 1024.0 * 1024.0;

    thread::spawn(move || {
        log::info!("[download] {model_id}: starting simulated download");

        // Initial delay before the first progress tick (mock: 150ms).
        thread::sleep(Duration::from_millis(150));

        let mut downloaded: f64 = 0.0;
        loop {
            downloaded += total / 12.0;
            let percentage = ((downloaded / total) * 100.0).round().min(100.0) as i64;
            let _ = app.emit(
                "model-download-progress",
                DownloadProgress {
                    model_id: model_id.clone(),
                    downloaded: downloaded.min(total),
                    total,
                    percentage,
                },
            );

            if percentage < 100 {
                thread::sleep(Duration::from_millis(220));
                continue;
            }

            // Verify -> extract -> complete (mock: 500ms gaps).
            let _ = app.emit("model-verification-started", &model_id);
            thread::sleep(Duration::from_millis(500));
            let _ = app.emit("model-verification-completed", &model_id);
            let _ = app.emit("model-extraction-started", &model_id);
            thread::sleep(Duration::from_millis(500));
            let _ = app.emit("model-extraction-completed", &model_id);

            if let Ok(mut state) = app.state::<Mutex<AppState>>().lock() {
                state.mark_downloaded(&model_id);
            }
            let _ = app.emit("model-download-complete", &model_id);
            log::info!("[download] {model_id}: complete");
            break;
        }
    });
}
