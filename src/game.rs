use sfml::{
  graphics::{Color, Rect, RenderTarget, RenderWindow, View},
  system::Vector2f,
  window::{mouse::Button, Event, Style},
};

use crate::{
  errors::MapLoadError, nation::types::Nations, player::types::Player, world_map::types::WorldMap,
};

pub struct Game {
  window: RenderWindow,
  world_map: WorldMap,
  player: Box<Player>,
}

const DEFAULT_MAP_PATH: &str = "./geo/countries.geojson";

impl Game {
  pub fn new(map_path: Option<String>) -> Result<Game, MapLoadError> {
    let map_path = if map_path.is_some() {
      map_path.unwrap()
    } else {
      DEFAULT_MAP_PATH.to_string()
    };
    let world_map = WorldMap::new(map_path)?;
    let mut window = RenderWindow::new((1920, 1080), "mapgame", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);
    let player = Player::new();
    Ok(Game {
      window,
      world_map,
      player,
    })
  }

  pub fn start(&mut self) {
    let window = &mut self.window;
    let world_map = &mut self.world_map;
    let player = &mut self.player;
    while window.is_open() {
      while let Some(event) = window.poll_event() {
        match event {
          Event::Closed => window.close(),
          Event::Resized { width, height } => {
            let bounds = Rect::new(0.0, 0.0, width as f32, height as f32);
            let nations = &mut world_map.nations;
            Game::on_resize(window, nations, bounds);
          }
          Event::MouseMoved { x, y } => {
            let position = Vector2f::new(x as f32, y as f32);
            let nations = &mut world_map.nations;
            let new_highlighted_id = WorldMap::get_highlighted_nation_at(nations, position);
            world_map.highlighted_nation_id = new_highlighted_id;
          }
          Event::MouseButtonPressed { button, x: _, y: _ } => {
            Game::on_mouse_button_press(button, world_map, player);
          }
          _ => {}
        }
      }
      window.clear(Color::WHITE);
      world_map.render(window);
      window.display();
    }
  }

  fn on_resize(window: &mut RenderWindow, nations: &mut Nations, bounds: Rect<f32>) {
    window.set_view(&View::from_rect(bounds));
    for (_id, nation) in nations.iter_mut() {
      nation.on_resize(bounds);
    }
  }

  fn on_mouse_button_press(button: Button, world_map: &mut WorldMap, player: &mut Box<Player>) {
    if button == Button::Left && player.nation_id.is_none() {
      let highlighted_id = &world_map.highlighted_nation_id;
      if highlighted_id.is_some() {
        player.nation_id = highlighted_id.clone();
        println!("SELECTED NATION: {}", player.nation_id.as_ref().unwrap());
      }
    }
  }
}
