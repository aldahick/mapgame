use geojson::{feature::Id, Feature, Value};
use sfml::{graphics::Rect, system::Vector2f};

use crate::{
  errors::MapLoadError,
  math::{polygon_area, polygon_contains},
};

pub type Bounds = Rect<f32>;
pub type GeoPolygons = Vec<Vec<Vec<Vec<f64>>>>;
pub type VectorPolygons = Vec<Vec<Vector2f>>;

pub const MAX_LATITUDE: f32 = 180.0;
pub const MAX_LONGITUDE: f32 = 90.0;

#[derive(Debug)]
pub struct GeoDrawable {
  pub id: String,
  pub name: String,
  pub geo_polygons: GeoPolygons,
  pub vector_polygons: VectorPolygons,
  pub vector_total_area: f32,
  pub bounds: Vec<Bounds>,
}

pub trait GeoDrawableUpdater {
  fn update_cached_vertices(&mut self);
}

impl GeoDrawable {
  pub fn new(
    feature: Feature,
    world_bounds: &Bounds,
    map_name: String,
    name_property: &str,
    id_property: Option<&str>,
  ) -> Result<Box<GeoDrawable>, MapLoadError> {
    let id = if id_property.is_none() {
      feature
        .id
        .clone()
        .and_then(|id| match id {
          Id::String(id) => Some(id),
          Id::Number(id) => Some(id.to_string()),
        })
        .ok_or_else(|| MapLoadError {
          name: map_name.clone(),
          reason: format!("failed to load province: no ID found"),
        })
    } else {
      GeoDrawable::get_feature_property(&feature, &map_name, id_property.unwrap())
    }?;
    let name = GeoDrawable::get_feature_property(&feature, &map_name, name_property)?;
    let geometry = feature.geometry.as_ref().ok_or_else(|| MapLoadError {
      name: map_name,
      reason: format!("failed to load province geometry for '{}'", name),
    })?;
    let mut geo_polygons: GeoPolygons = Vec::new();
    if let Value::Polygon(polygon) = geometry.value.clone() {
      geo_polygons.push(polygon);
    } else if let Value::MultiPolygon(multi) = geometry.value.clone() {
      geo_polygons.extend(multi);
    }
    let vector_polygons = GeoDrawable::to_vector_polygons(&geo_polygons, world_bounds);
    let vector_total_area = GeoDrawable::to_vector_total_area(&vector_polygons);
    let bounds = GeoDrawable::to_bounds(&vector_polygons);
    Ok(Box::new(GeoDrawable {
      id,
      name,
      geo_polygons,
      vector_polygons,
      vector_total_area,
      bounds,
    }))
  }

  pub fn get_feature_property(
    feature: &Feature,
    map_name: &String,
    key: &str,
  ) -> Result<String, MapLoadError> {
    Ok(
      feature
        .property(key)
        .ok_or_else(|| MapLoadError {
          name: map_name.clone(),
          reason: format!("failed to get property from GeoJSON feature: {}", key),
        })?
        .to_string(),
    )
  }

  pub fn on_resize(&mut self, bounds: &Bounds) {
    self.vector_polygons = GeoDrawable::to_vector_polygons(&self.geo_polygons, bounds);
    self.bounds = GeoDrawable::to_bounds(&self.vector_polygons);
  }

  pub fn includes(&self, point: Vector2f) -> bool {
    for bounds in &self.bounds {
      if bounds.contains(point) {
        for polygon in &self.vector_polygons {
          if polygon_contains(point, polygon) {
            return true;
          }
        }
      }
    }
    false
  }

  pub fn to_vector_total_area(vector_polygons: &VectorPolygons) -> f32 {
    let mut total_area = 0.0;
    for polygon in vector_polygons {
      total_area += polygon_area(polygon);
    }
    total_area
  }

  pub fn to_vector_polygons(polygons: &GeoPolygons, bounds: &Bounds) -> Vec<Vec<Vector2f>> {
    let mut vector_groups = Vec::new();
    for polygon in polygons {
      // see: https://stevage.github.io/geojson-spec/#section-3.1.6
      for linear_ring in polygon {
        let mut vector_group = Vec::new();
        if let Some((_last, points)) = linear_ring.as_slice().split_last() {
          for point in points {
            vector_group.push(GeoDrawable::to_vector(point, bounds));
          }
        }
        vector_groups.push(vector_group);
      }
    }
    vector_groups
  }

  pub fn to_vector(point: &Vec<f64>, bounds: &Bounds) -> Vector2f {
    let latitude = point[0] as f32;
    let longitude = point[1] as f32;
    let x = (latitude + MAX_LATITUDE) * bounds.width / (2.0 * MAX_LATITUDE);
    let y = bounds.height - ((longitude + MAX_LONGITUDE) * bounds.height / (2.0 * MAX_LONGITUDE));
    Vector2f::new(x, y)
  }

  pub fn to_bounds(vector_polygons: &VectorPolygons) -> Vec<Bounds> {
    let mut bounds = Vec::new();
    for polygon in vector_polygons {
      let mut min_x = f32::MAX;
      let mut min_y = f32::MAX;
      let mut max_x = 0.0;
      let mut max_y = 0.0;
      for vector in polygon {
        if min_x > vector.x {
          min_x = vector.x;
        }
        if max_x < vector.x {
          max_x = vector.x;
        }
        if min_y > vector.y {
          min_y = vector.y;
        }
        if max_y < vector.y {
          max_y = vector.y;
        }
      }
      bounds.push(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y));
    }
    bounds
  }
}
