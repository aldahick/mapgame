use std::error::Error;

use sfml::{
  graphics::{Color, Rect, RenderTarget, RenderWindow, View},
  system::Vector2f,
  window::{mouse::Button, Event, Style},
};

use crate::{config::Config, player::types::Player, world_map::types::WorldMap};

pub struct Game {
  window: RenderWindow,
  world_map: WorldMap,
  player: Box<Player>,
}

impl Game {
  pub fn new(config: &Config) -> Result<Game, Box<dyn Error>> {
    let world_map = WorldMap::new(config)?;
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
      Event::Closed => self.on_close(),
      Event::Resized { width, height } => {
        self.on_resize(Rect::new(0.0, 0.0, width as f32, height as f32))
      }
      Event::MouseMoved { x, y } => {
        self.on_mouse_move(Vector2f::new(x as f32, y as f32));
      }
      Event::MouseButtonPressed { button, x: _, y: _ } => {
        self.on_mouse_button_press(button);
      }
      _ => {}
    }
  }

  fn on_close(&mut self) {
    self.window.close();
  }

  fn on_resize(&mut self, bounds: Rect<f32>) {
    self.window.set_view(&View::from_rect(bounds));
    for (_id, nation) in self.world_map.nations.iter_mut() {
      nation.on_resize(bounds);
    }
  }

  fn on_mouse_move(&mut self, position: Vector2f) {
    if match self.world_map.get_highlighted_nation() {
      Some(n) => n.includes(position),
      None => false,
    } {
      return;
    }
    self.world_map.set_highlighted_nation_at(position);
  }

  fn on_mouse_button_press(&mut self, button: Button) {
    if button == Button::Left && self.player.nation_id.is_none() {
      let highlighted_nation = self.world_map.get_highlighted_nation();
      if highlighted_nation.is_some() {
        let old_selected_id = self.player.nation_id.clone();
        self.player.nation_id = highlighted_nation.and_then(|n| Some(n.id.clone()));
        let new_selected_id = self.player.nation_id.clone().unwrap();
        self
          .world_map
          .set_selected_nation(old_selected_id, new_selected_id);
      }
    }
  }
}
