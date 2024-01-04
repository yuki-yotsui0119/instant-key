mod icons;
mod preferences;

extern crate directories;
extern crate plist;

use directories::ProjectDirs;
use std::{process::Command, time::Instant};

pub use icons::convert_all_app_icons_to_png;
pub use preferences::create_preferences_if_missing;


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
pub fn open_command(path: &str) {
    Command::new("open")
        .arg(path.trim())
        .spawn()
        .expect("failed to execute process");
}
