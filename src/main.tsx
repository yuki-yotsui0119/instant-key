import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

import { appDataDir, join, resolveResource } from '@tauri-apps/api/path';
import { appWindow } from "@tauri-apps/api/window";
import { invoke, app } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { keyDatas, paths, preferences } from "./cache";
import { readTextFile } from "@tauri-apps/api/fs";
import { Key } from "../src-tauri/bindings/Key.ts";
import { Config } from "../src-tauri/bindings/Config.ts";

// await appWindow.center();
// await appWindow.maximize();
// await appWindow.setFocus();


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
