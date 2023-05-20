#![deny(elided_lifetimes_in_paths)]

pub mod errors;
pub mod game;
pub mod math;
pub mod nation;
pub mod player;
pub mod world_map;

use game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}
