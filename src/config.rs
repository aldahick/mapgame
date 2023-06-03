use std::{env, error::Error, path::Path};

use tokio::{
  fs::{create_dir, File},
  io::AsyncWriteExt,
};

pub struct Config {
  pub nations_path: String,
}

const NATIONS_PATH_KEY: &str = "NATIONS_PATH";
const DEFAULT_NATIONS_PATH: &str = "./mapgame/countries.geojson";
const DEFAULT_NATIONS_URL: &str =
  "https://s.ahicks.dev/complete-reflecting-zonetailedpigeon/direct";

impl Config {
  pub async fn new() -> Result<Config, Box<dyn Error>> {
    let nations_path = env::var(NATIONS_PATH_KEY).unwrap_or(DEFAULT_NATIONS_PATH.to_string());
    Config::download_file(DEFAULT_NATIONS_URL, &nations_path).await?;
    Ok(Config { nations_path })
  }

  async fn download_file(url: &str, path_str: &str) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(DEFAULT_NATIONS_URL).await?.bytes().await?;
    let path = Path::new(path_str);
    if path.exists() {
      return Ok(());
    }
    println!("Downloading {} to {}", url, path_str);
    create_dir(path.parent().unwrap()).await?;
    let mut file = File::create(&path).await?;
    file.write_all(&resp).await?;
    println!("Downloaded to {} successfully", path_str);
    Ok(())
  }
}
