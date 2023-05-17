use sfml::{graphics::{RenderWindow, View, Rect, RenderTarget, Color}, window::{Style, Event}};

use crate::worldmap::WorldMap;

pub struct Game {
  window: RenderWindow,
  world_map: WorldMap
}

const MAP_PATH: &str = "./geo/countries.geojson";

impl Game {
  pub fn new() -> Game {
    let world_map = WorldMap::new(MAP_PATH.to_string()).expect("Failed to load map from file");
    let mut window = RenderWindow::new((800, 600), "SFML window", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);
    Game { window, world_map }
  }

  pub fn start(&mut self) {
    while self.window.is_open() {
      while let Some(event) = self.window.poll_event() {
        self.on_event(event);
      }
      self.window.clear(Color::WHITE);
      self.world_map.render(&mut self.window);
      self.window.display();
    }
  }

  fn on_event(&mut self, event: Event) {
    match event {
      Event::Closed => self.window.close(),
      Event::Resized { width, height } => self.window.set_view(&View::from_rect(Rect::new(
        0.0,
        0.0,
        width as f32,
        height as f32,
      ))),
      _ => {}
    }
  }
}
