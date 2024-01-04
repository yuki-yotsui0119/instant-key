mod icons;
mod preferences;

extern crate directories;
extern crate plist;

use directories::ProjectDirs;
use tauri::command;
use std::{process::Command, time::Instant};

pub use icons::convert_all_app_icons_to_png;
pub use preferences::create_preferences_if_missing;
pub use preferences::get_all_config_data;

use crate::util::preferences::KeyType;
use crate::util::preferences::get_config_data;


#[tauri::command]
pub fn get_icon(app_name: &str) -> String {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "instant-key") {
        let icon_dir = proj_dirs.config_dir().join("appIcons");
        let icon_path = icon_dir.join(app_name.to_owned() + &".png");
        if icon_path.exists() {
            return icon_path.to_str().unwrap().to_owned();
        }
        return String::from("");
    }
    return String::from("");
}

#[tauri::command]
pub fn execute_command(key: &str) {
    if let Some(config_data) = get_config_data(key) {
        match config_data.key_type {
            KeyType::Command => {
                if let Some(command) = &config_data.command {
                    let mut command_parts = command.split_whitespace();
                    let command_name = match command_parts.next() {
                        Some(name) => name,
                        None => {
                            eprintln!("Command name is missing");
                            return;
                        }
                    };
                    let command_args = command_parts.collect::<Vec<&str>>();
                    println!("command_name: {:?}", command_name);
                    println!("command_args: {:?}", command_args);

                    if let Err(e) = Command::new(command_name).args(command_args).spawn() {
                        eprintln!("Failed to execute process: {}", e);
                    }
                } else {
                    eprintln!("Command data is missing");
                }
            },
            KeyType::Explorer => {
                if let Some(path) = &config_data.path {
                    // Windowsのエクスプローラーを指定のパスで開く
                    if let Err(e) = Command::new("explorer").arg(path).spawn() {
                        eprintln!("Failed to open explorer with path: {}, Error: {}", path, e);
                    }
                } else {
                    eprintln!("Path data is missing for Explorer");
                }
            }
        }
    } else {
        eprintln!("No configuration data found for key: {}", key);
    }
}


#[tauri::command]
pub fn open_command(path: &str) {
    Command::new("open")
        .arg(path.trim())
        .spawn()
        .expect("failed to execute process");
}
