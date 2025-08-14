#[cfg(target_os = "windows")]
#[link(name = "Advapi32")]
unsafe extern "system" {}

pub mod config;
pub mod errors;
pub mod game;
pub mod geo_drawable;
pub mod math;
pub mod nation;
pub mod player;
pub mod province;
pub mod world_map;

use config::get_config;
use game::Game;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let config = get_config()?;
  let mut game = Game::new(config)?;
  game.start();
  Ok(())
}
