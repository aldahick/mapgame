#![deny(elided_lifetimes_in_paths)]

pub mod errors;
pub mod game;
pub mod math;
pub mod nation;
pub mod player;
pub mod world_map;

use game::Game;

fn main() {
  let mut game = match Game::new(None) {
    Ok(g) => g,
    Err(e) => {
      println!("Failed to load the map!\n{}", e.reason);
      return;
    }
  };
  game.start();
}
