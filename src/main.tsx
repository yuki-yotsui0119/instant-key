import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

import { appDataDir, join, resolveResource } from '@tauri-apps/api/path';
import { appWindow } from "@tauri-apps/api/window";

// await appWindow.center();
// await appWindow.maximize();
// await appWindow.setFocus();


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
