use std::collections::HashMap;

use sfml::graphics::Vertex;

use crate::world_map::types::{Bounds, GeoPolygons, VectorPolygons};

#[derive(Debug)]
pub struct Nation {
  pub id: String,
  pub name: String,
  pub(crate) highlighted: bool,
  pub(crate) selected: bool,
  pub(crate) cached_vertices: Vec<Vec<Vertex>>,
  pub(crate) vector_polygons: VectorPolygons,
  pub(crate) geo_polygons: GeoPolygons,
  pub(crate) vector_total_area: f32,
  pub(crate) bounds: Vec<Bounds>,
}
pub(crate) type Nations = HashMap<String, Box<Nation>>;

impl Nation {
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
}
