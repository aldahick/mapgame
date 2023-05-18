use std::collections::HashMap;

use crate::world_map::types::{Bounds, GeoPolygons, VectorPolygons};

#[derive(Debug)]
pub struct Nation {
  pub id: String,
  pub name: String,
  pub(crate) vector_polygons: VectorPolygons,
  pub(crate) geo_polygons: GeoPolygons,
  pub(crate) vector_total_area: f32,
  pub(crate) bounds: Vec<Bounds>,
  pub(crate) highlight: bool,
}
pub(crate) type Nations = HashMap<String, Box<Nation>>;
