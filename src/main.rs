mod errors;
mod nation;
mod worldmap;

use sfml::graphics::{Color, Rect, RenderTarget, RenderWindow, View};
use sfml::window::{Event, Style};
use worldmap::WorldMap;

fn new_window() -> RenderWindow {
  let mut window = RenderWindow::new((800, 600), "SFML window", Style::CLOSE, &Default::default());
  window.set_framerate_limit(60);
  window
}

fn start(mut window: &mut RenderWindow, mut world_map: WorldMap) {
  // let view: &mut View = window.view();
  while window.is_open() {
    while let Some(event) = window.poll_event() {
      match event {
        Event::Closed => window.close(),
        Event::Resized { width, height } => window.set_view(&View::from_rect(Rect::new(
          0.0,
          0.0,
          width as f32,
          height as f32,
        ))),
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
  let mut window = new_window();
  let path = "geo/countries.geojson";
  let world_map = WorldMap::new(path.to_string()).expect("Failed to load map from file");
  start(&mut window, world_map);
}
