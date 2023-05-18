use std::{collections::HashMap};

use geojson::{Feature, Value};
use sfml::{
  graphics::{Color, Drawable, PrimitiveType, Vertex},
  system::Vector2f,
};

use crate::{
  errors::MapParseError,
  math::{polygon_contains, polygon_area},
  worldmap::{Bounds, GeoPolygons, VectorPolygons, WorldMap},
};

#[derive(Debug)]
pub struct Nation {
  pub id: String,
  pub name: String,
  geo_polygons: GeoPolygons,
  vector_polygons: VectorPolygons,
  vector_total_area: f32,
  bounds: Vec<Bounds>,
  highlight: bool,
}
pub type Nations = HashMap<String, Box<Nation>>;

impl Nation {
  pub fn new(feature: Feature, bounds: Bounds) -> Result<Box<Nation>, MapParseError> {
    let id = match feature.property("ISO_A3") {
      Some(v) => v.as_str().unwrap(),
      None => "",
    };
    let name = match feature.property("ADMIN") {
      Some(v) => v.as_str().unwrap(),
      None => "",
    };
    let geometry = match feature.geometry.as_ref().ok_or_else(|| MapParseError) {
      Ok(g) => g,
      Err(e) => return Err(e),
    };
    let mut geo_polygons: GeoPolygons = Vec::new();
    if let Value::Polygon(polygon) = geometry.value.clone() {
      geo_polygons.push(polygon);
    } else if let Value::MultiPolygon(multi) = geometry.value.clone() {
      geo_polygons.extend(multi);
    }
    let vector_polygons = WorldMap::to_vector_polygons(&geo_polygons, bounds);
    let mut vector_total_area = 0.0;
    for polygon in &vector_polygons {
      vector_total_area += polygon_area(polygon);
    }
    Ok(Box::new(Nation {
      id: id.to_string(),
      name: name.to_string(),
      bounds: WorldMap::to_bounds(&vector_polygons),
      vector_polygons,
      vector_total_area: f32::abs(vector_total_area / 2.0),
      geo_polygons,
      highlight: false,
    }))
  }

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
