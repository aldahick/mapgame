use sfml::{
  graphics::{Color, Drawable, PrimitiveType, Vertex},
  system::Vector2f,
};

use crate::{
  math::polygon_contains,
  world_map::types::{Bounds, WorldMap},
};

use super::types::Nation;

impl Nation {
  pub fn includes(&self, point: Vector2f) -> bool {
    let mut bounds_includes = false;
    for bounds in &self.bounds {
      if bounds.contains(point) {
        bounds_includes = true;
      }
    }
    if !bounds_includes {
      return false;
    }
    for polygon in &self.vector_polygons {
      if polygon_contains(point, polygon) {
        return true;
      }
    }
    false
  }

  pub fn area(&self) -> f32 {
    self.vector_total_area
  }

  pub fn set_highlight(&mut self, value: bool) {
    self.highlight = value;
  }

  pub fn on_resize(&mut self, bounds: Bounds) {
    self.vector_polygons = WorldMap::to_vector_polygons(&self.geo_polygons, bounds);
    self.bounds = WorldMap::to_bounds(&self.vector_polygons);
  }
}

impl Drawable for Nation {
  fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
    &'a self,
    target: &mut dyn sfml::graphics::RenderTarget,
    states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
  ) {
    let zero = Vector2f::new(0.0, 0.0);
    for vectors in &self.vector_polygons {
      let mut vertices = Vec::new();
      for vector in vectors {
        let color = if self.highlight {
          Color::GREEN
        } else {
          Color::BLACK
        };
        vertices.push(Vertex::new(*vector, color, zero));
      }
      target.draw_primitives(vertices.as_slice(), PrimitiveType::LINE_STRIP, states);
    }
  }
}
