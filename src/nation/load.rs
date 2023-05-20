use geojson::{Feature, Value};

use crate::{
  errors::MapLoadError,
  math::polygon_area,
  world_map::types::{Bounds, GeoPolygons, WorldMap},
};

use super::types::Nation;

impl Nation {
  pub fn new<'n>(
    feature: Feature,
    bounds: Bounds,
    map_name: String,
  ) -> Result<Box<Nation>, MapLoadError> {
    let id = feature
      .property("ISO_A3")
      .and_then(|p| Some(p.to_string()))
      .unwrap_or_default();
    let name = feature
      .property("ADMIN")
      .and_then(|p| Some(p.to_string()))
      .unwrap_or_default();
    let geometry = feature.geometry.as_ref().ok_or_else(|| MapLoadError {
      name: map_name,
      reason: format!("failed to parse geometry for nation '{}'", name),
    })?;
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
    let nation = Box::new(Nation {
      id: id.to_string(),
      name: name.to_string(),
      bounds: WorldMap::to_bounds(&vector_polygons),
      vector_polygons,
      vector_total_area: f32::abs(vector_total_area / 2.0),
      geo_polygons,
      highlight: false,
    });
    Ok(nation)
  }
}
