use sfml::{
  graphics::{Color, Rect, RenderTarget, RenderWindow, View},
  system::Vector2f,
  window::{Event, Style},
};

use crate::{player::types::Player, world_map::types::WorldMap};

pub struct Game {
  window: RenderWindow,
  world_map: WorldMap,
  player: Box<Player>,
}

const MAP_PATH: &str = "./geo/countries.geojson";

impl Game {
  pub fn new() -> Game {
    let world_map = WorldMap::new(MAP_PATH.to_string()).expect("Failed to load map from file");
    let mut window = RenderWindow::new((1920, 1080), "mapwar", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);
    let player = Player::new();
    Game {
      window,
      world_map,
      player,
    }
  }

  pub fn start(&mut self) {
    let window = &mut self.window;
    let world_map = &mut self.world_map;
    while window.is_open() {
      while let Some(event) = window.poll_event() {
        match event {
          Event::Closed => window.close(),
          Event::Resized { width, height } => {
            let bounds = Rect::new(0.0, 0.0, width as f32, height as f32);
            window.set_view(&View::from_rect(bounds.as_other::<f32>()));
            let nations = &mut world_map.nations;
            for (_id, nation) in nations.iter_mut() {
              nation.on_resize(bounds);
            }
          }
          Event::MouseMoved { x, y } => {
            let position = Vector2f::new(x as f32, y as f32);
            let nations = &mut world_map.nations;
            let new_highlighted_id = WorldMap::get_highlighted_nation_at(nations, position);
            world_map.highlighted_nation_id = new_highlighted_id;
          }
          Event::MouseButtonPressed {
            button: _,
            x: _,
            y: _,
          } => {
            if self.player.nation_id.is_none() {
              let highlighted_id = &world_map.highlighted_nation_id;
              if highlighted_id.is_some() {
                self.player.nation_id = highlighted_id.clone();
                println!(
                  "SELECTED NATION: {}",
                  self.player.nation_id.as_ref().unwrap()
                );
              }
            }
          }
          _ => {}
        }
      }
      window.clear(Color::WHITE);
      world_map.render(window);
      window.display();
      self.player = Player::new();
    }
  }
}
