#![deny(elided_lifetimes_in_paths)]

pub mod config;
pub mod errors;
pub mod game;
pub mod geo_drawable;
pub mod math;
pub mod nation;
pub mod player;
pub mod province;
pub mod util;
pub mod world_map;

use std::error::Error;

use config::Config;
use game::Game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let config = Config::new().await?;
  let mut game = Game::new(&config).await?;
  game.start();
  Ok(())
}
