pub mod errors;
pub mod game;
pub mod math;
pub mod nation;
pub mod world_map;

use game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}
