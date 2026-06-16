import React from "react";
import ReactDOM from "react-dom/client";
import RecordingOverlay from "./RecordingOverlay";
import "@/i18n";
// Seed the mock event bus so the overlay demonstrates its recording state.
import "./demo";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RecordingOverlay />
  </React.StrictMode>,
);
