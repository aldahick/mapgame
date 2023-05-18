use sfml::{
  graphics::{Color, Rect, RenderTarget, RenderWindow, View},
  system::Vector2f,
  window::{Event, Style},
};

use crate::worldmap::WorldMap;

pub struct Game {
  window: RenderWindow,
  world_map: WorldMap,
}

const MAP_PATH: &str = "./geo/countries.geojson";

impl Game {
  pub fn new() -> Game {
    let world_map = WorldMap::new(MAP_PATH.to_string()).expect("Failed to load map from file");
    let mut window = RenderWindow::new((1920, 1080), "mapwar", Style::CLOSE, &Default::default());
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
      Event::Resized { width, height } => self.on_resize(width as f64, height as f64),
      Event::MouseMoved { x, y } => self
        .world_map
        .set_highlight(Vector2f::new(x as f32, y as f32)),
      _ => {}
    }
  }

  fn on_resize(&mut self, width: f64, height: f64) {
    let bounds = Rect::new(0.0, 0.0, width as f32, height as f32);
    self
      .window
      .set_view(&View::from_rect(bounds.as_other::<f32>()));
    for (_id, nation) in self.world_map.nations.iter_mut() {
      nation.on_resize(bounds);
    }
  }
}
