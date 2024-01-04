// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use util::{convert_all_app_icons_to_png};
use window_vibrancy::{apply_acrylic};
use tauri::{
    Manager, GlobalShortcutManager
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    convert_all_app_icons_to_png();
    // create_preferences_if_missing();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // open_command,
            // get_icon,
            // handle_input,
            // launch_on_login,
            // ns_panel::init_ns_panel,
            // ns_panel::show_app,
            // ns_panel::hide_app
        ])
        .setup(|app| {
            // app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "windows")]
            apply_acrylic(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            window.hide().unwrap();

            window.center().unwrap();

            // ショートカット"Ctrl+Shift+Space"を登録
            app.global_shortcut_manager()
                .register("Ctrl+Alt+S", move || {
                    // ウィンドウの表示状態を切り替える
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap(); // ウィンドウにフォーカスを当てる
                    }
                })
                .expect("failed to register global shortcut");



            Ok(())
        })
        // .manage(ns_panel::State::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

