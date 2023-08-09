use std::{
    fs,
    println,
    path::PathBuf,
};

use reqwest::{
    StatusCode,
    Client,
};

use serde_json::Value;
use url::Url;

struct Download {
    url: String,
    path: PathBuf,
}

pub struct Downloader {
    client: Client,
    queue: Vec<Download>
}

impl Downloader {
    pub fn new() -> Self {
        let client = Client::new();
        let queue: Vec<Download> = Vec::new();
        Self {
            client,
            queue,
        }
    }

    pub fn push_to_queue(&mut self, url: String, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let download = Download {
            url,
            path,
        };

        self.queue.push(download);
        Ok(())
    }

    pub async fn download_file(&self, url: String, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        for _n in (1..10).rev() {
            let name = match Url::parse(&url) {
                Ok(result) => result,
                Err(err) => {
                    println!("Failed to parse URL: {}", err);
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue
                }
            };

            let name = match name.path_segments() {
                Some(result) => result,
                None => { 
                    println!("Failed to parse URL, retrying...");
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue 
                } 
            };
            
            let name = match name.last() {
                Some(result) => result,
                None => {
                    println!("Failed to parse name from URL, retrying...");
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue
                }
            };

            let mut full_path = PathBuf::from(path.clone());
            full_path.push(name);

            if full_path.is_file() {
                println!("File already downloaded, skipping...");
                break
            }

            let response = match self.client.get(&url).send().await {
                Ok(resp) => resp,
                Err(err) => {
                    println!("Error while sending download request: {}", err);
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue
                }
            };

            if response.status() != StatusCode::OK {
                println!("The request didn't respond with the correct status code: {}", response.status());
                std::thread::sleep(std::time::Duration::from_secs(3));
                continue
            }

            let body = match response.bytes().await {
                Ok(resp) => resp,
                Err(err) => {
                    println!("Failed to convert response to bytes, retrying... : {} ", err);
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue 
                }
            };

            if !path.exists() {
                fs::create_dir_all(path) 
                    .expect("Failed to create destination directory");
            }

            fs::write(&full_path, &body)
                .expect("Failed to create file");

            println!("{} downloaded", &full_path.to_str().expect("Filename contains invalid unicode (failed to convert path to str)"));

            break
        }

        Ok(())
    }

    pub async fn to_text(&self, url: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut text: String = String::new();

        for _n in (0..10).rev() {
            let response = match self.client.get(&url).send().await {
                Ok(resp) => resp,
                Err(err) => {
                    println!("Error requesting text: {}", err);
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue
                }
            };
            
            let response = match response.text().await {
                Ok(resp) => resp,
                Err(err) => {
                    println!("Error converting request to text: {}", err);
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue
                }
            };

            text = response;
        }

        Ok(text)
    }

    pub async fn to_json(&self, url: String) -> Result<Value, Box<dyn std::error::Error>> {
        let mut json: Value = Value::Null;
        for _n in (0..10).rev() {
            let response = match self.client.get(&url).send().await {
                Ok(resp) => resp,
                Err(err) => {
                    println!("Error requesting text: {}", err);
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue
                }
            };
            
            let response = match response.json().await {
                Ok(resp) => resp,
                Err(err) => {
                    println!("Error converting request to json: {}", err);
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue
                }
            };

            json = response;
        }

        Ok(json)
    }

    pub async fn download_queue(&mut self) {
        loop {
            match self.queue.pop() {
                Some(file) => self.download_file(file.url, &file.path).await
                    .expect("Failed to download file from queue"),
                None => {
                    println!("Empty queue");
                    break
                }
            }
        }
    }
}
