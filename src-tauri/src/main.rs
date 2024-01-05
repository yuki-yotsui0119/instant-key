// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use util::{convert_all_app_icons_to_png, create_preferences_if_missing, get_all_config_data, execute_command};
use window_vibrancy::apply_acrylic;
use tauri::{
    Manager, GlobalShortcutManager, WindowBuilder, Size, LogicalSize
};

fn main() {
    convert_all_app_icons_to_png();
    create_preferences_if_missing();
    get_all_config_data();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_config_data,
            execute_command,
        ])
        .setup(|app| {
            // app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let window = app.get_window("main").unwrap();


           // Retrieve the primary monitor's scale factor
           let primary_monitor = window.primary_monitor().unwrap().unwrap();
           let monitor_scale_factor = primary_monitor.scale_factor();
           let monitor_size = primary_monitor.size();

           // Calculate 70% of the current screen size
           let width = monitor_size.width as f64 * 0.8 / monitor_scale_factor;
           let height = monitor_size.height as f64 * 0.7 / monitor_scale_factor;

           // Set the main window size
           window.set_size(Size::Logical(LogicalSize::new(width, height))).unwrap();

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

