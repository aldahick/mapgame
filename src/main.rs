#![deny(elided_lifetimes_in_paths)]

pub mod errors;
pub mod game;
pub mod math;
pub mod nation;
pub mod player;
pub mod world_map;

use std::{env, error::Error, fs::File, io::Write, path::Path};

use game::Game;
use tokio::fs::create_dir;

const DEFAULT_MAP_URL: &str = "https://s.ahicks.dev/complete-reflecting-zonetailedpigeon/direct";
const DEFAULT_MAP_PATH: &str = "./mapgame/countries.geojson";

async fn get_map_path() -> Result<String, Box<dyn Error>> {
  let map_path = env::var("MAP_PATH").unwrap_or(DEFAULT_MAP_PATH.to_string());
  let path = Path::new(map_path.as_str());
  if path.exists() {
    return Ok(map_path);
  }
  println!("Downloading map data...");
  let resp = reqwest::get(DEFAULT_MAP_URL).await?.bytes().await?;
  create_dir(path.parent().unwrap()).await?;
  let mut file = File::create(&map_path).unwrap();
  match file.write_all(&resp) {
    Ok(it) => it,
    Err(err) => return Err(Box::new(err)),
  };
  println!("Done downloading map data");
  Ok(map_path)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let map_path = get_map_path().await?;
  let mut game = Game::new(map_path)?;
  game.start();
  Ok(())
}
