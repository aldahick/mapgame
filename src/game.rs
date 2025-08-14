use crate::{config::Config, player::Player, world_map::WorldMap};
use sfml::{
  SfResult,
  cpp::FBox,
  graphics::{Color, Rect, RenderTarget, RenderWindow, View},
  system::Vector2f,
  window::{Event, Style, mouse::Button},
};
use std::error::Error;

pub struct Game {
  config: Config,
  window: FBox<RenderWindow>,
  world_map: Box<WorldMap>,
  player: Box<Player>,
}

impl Game {
  pub fn new(config: Config) -> Result<Game, Box<dyn Error>> {
    let mut world_map = Box::new(WorldMap::new(&config.map)?);
    let mut window = RenderWindow::new((1920, 1080), "mapgame", Style::CLOSE, &Default::default())?;
    window.set_framerate_limit(60);
    let player = Player::new();
    let size = Rect::new(0f32, 0f32, 1920f32, 1080f32);
    world_map.on_resize(&size);
    Ok(Game {
      config,
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
      Event::Resized { width, height } => self
        .on_resize(Rect::new(0.0, 0.0, width as f32, height as f32))
        .unwrap_or(()),
      Event::MouseMoved { x, y } => {
        self.on_mouse_move(Vector2f::new(x as f32, y as f32));
      }
      Event::MouseButtonPressed { button, x, y } => {
        self.on_mouse_button_press(button, Vector2f::new(x as f32, y as f32));
      }
      Event::MouseWheelScrolled { delta, x, y, .. } => {
        self.on_mouse_wheel_scroll(delta, Vector2f::new(x as f32, y as f32));
      }
      _ => {}
    }
  }

  fn on_close(&mut self) {
    self.window.close();
  }

  fn on_resize(&mut self, bounds: Rect<f32>) -> SfResult<()> {
    let view = View::from_rect(bounds)?;
    self.window.set_view(&view);
    self.world_map.on_resize(&bounds);
    Ok(())
  }

  fn on_mouse_move(&mut self, position: Vector2f) {
    if !self.window.has_focus() {
      return;
    }
    if match self.world_map.get_highlighted_nation() {
      Some(n) => n.includes(position),
      None => false,
    } {
      return;
    }
    self.world_map.set_highlighted_nation_at(position);
  }

  fn on_mouse_button_press(&mut self, button: Button, position: Vector2f) {
    if button == Button::Left && self.player.nation_id.is_none() {
      let new_nation_id = self.world_map.set_selected_nation_at(position);
      self.player.nation_id = new_nation_id;
    }
  }

  fn on_mouse_wheel_scroll(&mut self, delta: f32, _position: Vector2f) {
    self.world_map.zoom = f32::max(self.config.view.min_zoom, self.world_map.zoom + delta * 0.1);
  }
}
