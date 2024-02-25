use tokio::{fs::OpenOptions, io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:12345";
    let listener = TcpListener::bind(address).await?;
    if let Err(e) = std::fs::create_dir("LOGS") {
        eprintln!("Ошибка при создании каталога: {}", e);
    }
    println!("Сервер запущен и ожидает подключения...");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let path_str = format!("LOGS/{}.zip", generate_random_filename());
            let mut file = match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .read(true)
                .open(&path_str)
                .await {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Ошибка при открытии файла: {}", e);
                        return;
                    }
                };
            while let Some(bytes) = read_next_bytes(&mut socket).await {
                if let Err(e) = file.write_all(&bytes).await {
                    eprintln!("Ошибка при записи в файл: {}", e);
                    return;
                }
                println!("Опа мамонт попался: {}", path_str);
            }
            if let Err(e) = file.sync_all().await {
                eprintln!("Ошибка при синхронизации файла: {}", e);
            }
        });
    }
}

async fn read_next_bytes(socket: &mut TcpStream) -> Option<Vec<u8>> {
    let mut buffer = Vec::new();
    match socket.read_to_end(&mut buffer).await {
        Ok(n) if n == 0 => None,
        Ok(_) => Some(buffer),
        Err(e) => {
            eprintln!("Ошибка при чтении данных из сокета: {}", e);
            None
        }
    }
}
fn generate_random_filename() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";

    let mut rng = thread_rng();
    let filename: String = (0..20)
        .map(|_| {
            *CHARSET.choose(&mut rng).expect("Failed to select random character") as char
        })
        .collect();

    filename
}