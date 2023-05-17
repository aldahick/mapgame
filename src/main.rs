mod errors;
mod nation;
mod worldmap;

use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};
use worldmap::WorldMap;

fn new_window() -> RenderWindow {
  let mut window = RenderWindow::new((800, 600), "SFML window", Style::CLOSE, &Default::default());
  window.set_framerate_limit(60);
  window
}

fn start(mut window: RenderWindow, mut world_map: WorldMap) {
  while window.is_open() {
    while let Some(event) = window.poll_event() {
      match event {
        Event::Closed => window.close(),
        _ => {}
      }
    }

    window.clear(Color::WHITE);
    // draw
    world_map.render(&mut window);
    window.display();
  }
}

fn main() {
  let window = new_window();
  let path = "geo/countries.geojson";
  let world_map = WorldMap::new(path.to_string()).expect("Failed to load map from file");
  start(window, world_map);
}
