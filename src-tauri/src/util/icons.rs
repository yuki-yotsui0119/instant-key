
use directories::ProjectDirs;
use std::{
    fs::{self},
    path::{Path, PathBuf}, // Add the missing import for PathBuf
    process::Command,
};

pub fn convert_all_app_icons_to_png() {
    let username = std::env::var("USERNAME").unwrap_or_default();
    let user_path = format!("C:\\Users\\{}\\AppData\\Local", username);
    let programs_path = format!("C:\\Users\\{}\\AppData\\Local\\Programs", username);

    let search_locations = vec![
        PathBuf::from("C:\\Program Files"),
        PathBuf::from("C:\\Program Files (x86)"),
        PathBuf::from(&programs_path),
        PathBuf::from(&user_path),
    ];

    for location in search_locations {
        process_directory(&location, 2); // depth is 2 for two levels down
    }
}

fn process_directory(dir: &Path, depth: usize) {
    if depth == 0 {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            process_directory(&path, depth - 1); // Recursive call
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("exe") {
            process_file(&path);
        }
    }
}

fn process_file(file_path: &Path) {
    let app_name = file_path.file_stem().unwrap().to_str().unwrap();
    let proj_dirs = ProjectDirs::from("", "", "instant-key").unwrap();
    let icon_dir = proj_dirs.config_dir().join("appIcons");
    ensure_directory_exists(&icon_dir);
    let icon_path = icon_dir.join(format!("{}.png", app_name));

    if !icon_path.exists() {
        save_icon(&file_path, &icon_path);
    }
}

fn ensure_directory_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create directory");
    }
}

fn save_icon(exe_path: &Path, icon_path: &Path) {
    let ps_script = format!(
        "[void][Reflection.Assembly]::LoadWithPartialName('System.Drawing');\
        [Drawing.Icon]::ExtractAssociatedIcon(\"{}\").ToBitmap().Save(\"{}\")",
        exe_path.display(),
        icon_path.display()
    );

    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &ps_script,
        ])
        .output()
        .expect("failed to execute process");

    if !output.stderr.is_empty() {
        eprintln!("Error saving icon for {}: {}", exe_path.display(), String::from_utf8_lossy(&output.stderr));
    }
}


        // match ImageReader::open(&ico_path) {
        //     Ok(reader) => match reader.decode() {
        //         Ok(img) => {
        //             let png_out = ico_path.with_extension("png");
        //             if let Err(e) = img.save(png_out) {
        //                 eprintln!("Failed to save PNG file: {}", e);
        //             }
        //         },
        //         Err(e) => eprintln!("Failed to decode ICO: {}", e),
        //     },
        //     Err(e) => eprintln!("Failed to open ICO file: {}", e),
        // }