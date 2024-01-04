// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use util::{convert_all_app_icons_to_png, create_preferences_if_missing, get_all_config_data, execute_command};
use window_vibrancy::apply_acrylic;
use tauri::{
    Manager, GlobalShortcutManager
};

fn main() {
    convert_all_app_icons_to_png();
    create_preferences_if_missing();
    get_all_config_data();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_config_data,
            execute_command,
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
        // .on_window_event(|event| match event.event() {
        //     tauri::WindowEvent::Focused(is_focused) => {
        //         // detect click outside of the focused window and hide the app
        //         if !is_focused {
        //             event.window().hide().unwrap();
        //         }
        //     }
        //     _ => {}
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

