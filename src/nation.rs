use std::collections::HashMap;

use geojson::Feature;
use sfml::{
  graphics::{Color, Drawable, PrimitiveType, Vertex},
  system::Vector2f,
};

use crate::{
  errors::MapLoadError,
  geo_drawable::{Bounds, GeoDrawable},
};

#[derive(Debug)]
pub struct Nation {
  pub highlighted: bool,
  pub selected: bool,
  pub geo_drawable: Box<GeoDrawable>,
  pub cached_vertices: Vec<Vec<Vertex>>,
}
pub type Nations = HashMap<String, Box<Nation>>;

impl Nation {
  pub fn new(
    feature: Feature,
    bounds: &Bounds,
    map_name: String,
  ) -> Result<Box<Nation>, MapLoadError> {
    let geo_drawable = GeoDrawable::new(feature, bounds, map_name, "ADMIN", Some("ISO_A3"))?;
    let mut nation = Box::new(Nation {
      geo_drawable,
      cached_vertices: Vec::new(),
      highlighted: false,
      selected: false,
    });
    nation.update_cached_vertices();
    Ok(nation)
  }

  pub fn is_highlighted(&self) -> bool {
    self.highlighted
  }

  pub fn is_selected(&self) -> bool {
    self.selected
  }

  pub fn set_highlighted(&mut self, value: bool) {
    self.highlighted = value;
    self.update_cached_vertices();
  }

  pub fn set_selected(&mut self, value: bool) {
    self.selected = value;
    self.update_cached_vertices();
  }

  pub fn area(&self) -> f32 {
    self.geo_drawable.vector_total_area
  }

  pub fn id(&self) -> &String {
    &self.geo_drawable.id
  }

  pub fn includes(&self, position: Vector2f) -> bool {
    self.geo_drawable.includes(position)
  }

  pub fn on_resize(&mut self, bounds: &Bounds) {
    self.geo_drawable.on_resize(bounds);
    self.update_cached_vertices();
  }

  pub fn update_cached_vertices(&mut self) {
    let zero = Vector2f::new(0.0, 0.0);
    let mut cached_vertices = Vec::new();
    for vectors in &self.geo_drawable.vector_polygons {
      let mut vertices = Vec::new();
      for vector in vectors {
        let color = if self.is_selected() {
          Color::BLUE
        } else if self.is_highlighted() {
          Color::GREEN
        } else {
          Color::BLACK
        };
        vertices.push(Vertex::new(*vector, color, zero));
      }
      cached_vertices.push(vertices);
    }
    self.cached_vertices = cached_vertices;
  }
}

impl Drawable for Nation {
  fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
    &'a self,
    target: &mut dyn sfml::graphics::RenderTarget,
    states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
  ) {
    for vertices in &self.cached_vertices {
      target.draw_primitives(vertices, PrimitiveType::LINE_STRIP, states);
    }
  }
}
