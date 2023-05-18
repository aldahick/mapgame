use geojson::{Feature, Value};

use crate::{
  errors::MapParseError,
  math::polygon_area,
  world_map::types::{Bounds, GeoPolygons, WorldMap},
};

use super::types::Nation;

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
}
