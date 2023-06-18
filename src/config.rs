use std::{env, error::Error};

use crate::util::download_file;

pub struct Config {
  pub nations_path: String,
  pub provinces_dir: String,
}

const NATIONS_PATH_KEY: &str = "NATIONS_PATH";
const DEFAULT_NATIONS_PATH: &str = "./mapgame/countries.geojson";
const DEFAULT_NATIONS_URL: &str = "https://s.ahicks.dev/nimble-mistyrose-locust/direct";

const PROVINCES_DIR_KEY: &str = "PROVINCES_DIR";
const DEFAULT_PROVINCES_DIR: &str = "./mapgame/provinces";

impl Config {
  pub async fn new() -> Result<Config, Box<dyn Error>> {
    let nations_path = env::var(NATIONS_PATH_KEY).unwrap_or(DEFAULT_NATIONS_PATH.to_string());
    let provinces_dir = env::var(PROVINCES_DIR_KEY).unwrap_or(DEFAULT_PROVINCES_DIR.to_string());
    download_file(DEFAULT_NATIONS_URL, &nations_path).await?;
    Ok(Config {
      nations_path,
      provinces_dir,
    })
  }
}
