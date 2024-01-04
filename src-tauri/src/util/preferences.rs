use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use ts_rs::TS;

// Typeを表す列挙型
#[derive(Serialize, Deserialize, Debug, TS, PartialEq)]
#[ts(export)]
pub enum KeyType {
    Command,
    Explorer,
}

// JSONデータの構造を表す構造体
#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Key {
    pub key: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "keyType")]
    pub key_type: KeyType,
    pub path: Option<String>,
    pub command: Option<String>, // コマンドは必ずしも存在しないのでOption型を使用
    #[serde(rename = "imagePath")]
    pub image_path: Option<String>
}

// 複数のKeyを持つことができる外側の構造体
#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Config {
    #[serde(default)] // デフォルト値を設定するためのアトリビュート
    keys: Vec<Key>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
struct Preferences {
    shortcut: String,
    launch_on_login: bool,
    menu_bar_icon: bool,
}

pub fn create_preferences_if_missing() {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "instant-key") {
        let preferences_path = proj_dirs.config_dir().join("preferences.json");
        let config_path = proj_dirs.config_dir().join("config.json");
        if !preferences_path.exists() {
            let preference = Preferences {
                shortcut: String::from("Command+Shift+G"),
                launch_on_login: true,
                menu_bar_icon: true,
            };
            let preference_text = serde_json::to_string(&preference).unwrap();
            fs::write(preferences_path, &preference_text).unwrap();
        }
        if !config_path.exists() {
            let mut config = Config { keys: Vec::new() };
            let config_text = serde_json::to_string(&config).unwrap();
            fs::write(config_path, &config_text).unwrap();
        }
    }
}

pub fn get_config_data(key: &str) -> Option<Key> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "instant-key") {
        let config_path = proj_dirs.config_dir().join("config.json");
        if config_path.exists() {
            let config_text = fs::read_to_string(config_path).unwrap();
            let config: Config = serde_json::from_str(&config_text).unwrap();
            for key_data in config.keys {
                if key_data.key == key {
                    return Some(key_data);
                }
            }
        }
    }
    return None;
}

#[tauri::command]
pub fn get_all_config_data() -> Vec<Key> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "instant-key") {
        let config_path = proj_dirs.config_dir().join("config.json");
        if config_path.exists() {
            let config_text = fs::read_to_string(config_path).unwrap();
            let config: Config = serde_json::from_str(&config_text).unwrap();
            return config.keys;
        }
    }
    return Vec::new();
}
