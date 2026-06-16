/**
 * Demo seed for the recording overlay.
 *
 * The real backend emits `show-overlay` and `mic-level` events while recording.
 * With no backend, we drive the same mock event bus so the overlay visibly
 * demonstrates its recording state with animated audio bars.
 */
import { emit } from "@tauri-apps/api/event";

let phase = 0;

emit("show-overlay", "recording");

setInterval(() => {
  phase += 0.18;
  const levels = Array.from({ length: 16 }, (_, i) => {
    const wave = 0.5 + 0.5 * Math.sin(phase + i * 0.6);
    const jitter = 0.5 + 0.5 * Math.sin(phase * 1.7 + i * 1.3);
    return Math.max(0.05, Math.min(1, wave * jitter));
  });
  emit("mic-level", levels);
}, 90);
