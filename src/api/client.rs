use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::path::PathBuf;

pub struct ApiClient {
    client: Client,
    cache_dir: PathBuf,
}

impl ApiClient {
    pub fn new() -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("pokemon-tui")
            .join("api");
        let _ = std::fs::create_dir_all(&cache_dir);
        Self {
            client: Client::new(),
            cache_dir,
        }
    }

    /// Fetch JSON, using filesystem cache
    pub async fn get_cached<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let cache_key = Self::url_to_cache_key(url);
        let cache_path = self.cache_dir.join(&cache_key);

        if let Ok(data) = std::fs::read_to_string(&cache_path) {
            if let Ok(parsed) = serde_json::from_str(&data) {
                return Ok(parsed);
            }
        }

        let resp = self.client.get(url).send().await?.text().await?;
        let _ = std::fs::write(&cache_path, &resp);
        Ok(serde_json::from_str(&resp)?)
    }

    /// Fetch raw bytes (for sprites), using filesystem cache
    pub async fn get_bytes_cached(&self, url: &str) -> Result<Vec<u8>> {
        let cache_key = Self::url_to_cache_key(url);
        let cache_path = self.cache_dir.join(&cache_key);

        if let Ok(data) = std::fs::read(&cache_path) {
            return Ok(data);
        }

        let bytes = self.client.get(url).send().await?.bytes().await?.to_vec();
        let _ = std::fs::write(&cache_path, &bytes);
        Ok(bytes)
    }

    fn url_to_cache_key(url: &str) -> String {
        url.replace("https://", "")
            .replace("http://", "")
            .replace('/', "_")
            .replace('?', "_")
    }
}
