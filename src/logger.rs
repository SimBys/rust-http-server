use std::error::Error;
use std::sync::Arc;
use chrono::Local;
use tokio::sync::Mutex;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct Logger{
    file: Arc<Mutex<File>>,
}

impl Logger {
    pub async fn new(log_file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .await?;

        Ok(Self {
            file: Arc::new(Mutex::new(file)),
        })
    }

    pub async fn log(&self, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_line = format!("[{}] {}", timestamp, message);

        let mut file = self.file.lock().await;
        if let Err(e) = file.write_all(log_line.as_bytes()).await {
            eprintln!("[!] Failed to write to log file: {}", e);
        }
    }

    pub fn clone(&self) -> Self {
        Logger {
            file: Arc::clone(&self.file),
        }
    }
}