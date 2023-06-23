#![deny(elided_lifetimes_in_paths)]

pub mod config;
pub mod errors;
pub mod game;
pub mod geo_drawable;
pub mod math;
pub mod nation;
pub mod player;
pub mod province;
pub mod world_map;

use std::error::Error;

use config::get_available_maps;
use errors::MapLoadError;
use game::Game;

fn main() -> Result<(), Box<dyn Error>> {
  let maps = get_available_maps()?;
  if maps.len() != 1 {
    panic!("Only one map is currently supported. lol");
  }
  let map_name = maps.keys().next().ok_or_else(|| MapLoadError {
    reason: "No maps found".to_string(),
  })?;
  let map = maps.get(map_name).ok_or_else(|| MapLoadError {
    reason: format!("Map {} not found", map_name),
  })?;
  let mut game = Game::new(map)?;
  game.start();
  Ok(())
}
