mod errors;
mod game;
mod nation;
mod worldmap;

use game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}
