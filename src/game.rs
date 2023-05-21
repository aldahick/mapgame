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

impl Game {
  pub fn new(map_path: String) -> Result<Game, MapLoadError> {
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
    let player = &mut self.player;
    let world_map = &mut self.world_map;
    while window.is_open() {
      while let Some(event) = window.poll_event() {
        let nations = &mut world_map.nations;
        match event {
          Event::Closed => window.close(),
          Event::Resized { width, height } => {
            let bounds = Rect::new(0.0, 0.0, width as f32, height as f32);
            Game::on_resize(window, nations, bounds);
          }
          Event::MouseMoved { x, y } => {
            let position = Vector2f::new(x as f32, y as f32);
            let new_highlighted_id = WorldMap::set_highlighted_nation_at(nations, position);
            world_map.highlighted_nation_id = new_highlighted_id.cloned();
          }
          Event::MouseButtonPressed { button, x: _, y: _ } => {
            Game::on_mouse_button_press(
              button,
              nations,
              &mut world_map.highlighted_nation_id,
              player,
            );
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

  fn on_mouse_button_press(
    button: Button,
    nations: &mut Nations,
    highlighted_nation_id: &mut Option<String>,
    player: &mut Box<Player>,
  ) {
    if button == Button::Left && player.nation_id.is_none() {
      if highlighted_nation_id.is_some() {
        let old_selected_id = player.nation_id.clone();
        player.nation_id = highlighted_nation_id.clone();
        let new_selected_id = highlighted_nation_id.clone().unwrap();
        WorldMap::set_selected_nation(nations, old_selected_id, new_selected_id);
      }
    }
  }
}
