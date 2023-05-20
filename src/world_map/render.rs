use std::ops::Deref;

use sfml::{
  graphics::{Rect, RenderTarget, RenderWindow},
  system::Vector2f,
};

use super::types::{Bounds, GeoPolygons, VectorPolygons, WorldMap, MAX_LATITUDE, MAX_LONGITUDE};

impl WorldMap {
  pub fn to_vector_polygons(polygons: &GeoPolygons, bounds: Bounds) -> Vec<Vec<Vector2f>> {
    let mut vector_groups = Vec::new();
    for polygon in polygons {
      // see: https://stevage.github.io/geojson-spec/#section-3.1.6
      for linear_ring in polygon {
        let mut vector_group = Vec::new();
        if let Some((_last, points)) = linear_ring.as_slice().split_last() {
          for point in points {
            vector_group.push(WorldMap::to_vector(point, bounds));
          }
        }
        vector_groups.push(vector_group);
      }
    }
    vector_groups
  }

  pub fn to_vector(point: &Vec<f64>, bounds: Bounds) -> Vector2f {
    let latitude = point[0] as f32;
    let longitude = point[1] as f32;
    let x = (latitude + MAX_LATITUDE) * bounds.width / (2.0 * MAX_LATITUDE);
    let y = bounds.height - ((longitude + MAX_LONGITUDE) * bounds.height / (2.0 * MAX_LONGITUDE));
    Vector2f::new(x, y)
  }

  pub fn render(&self, window: &mut RenderWindow) {
    let highlight_id = self.highlighted_nation_id.clone().unwrap_or_default();
    for (id, nation) in &self.nations {
      if id.as_str() != highlight_id {
        window.draw(nation.deref());
      }
    }
    let highlight_nation = self.nations.get(highlight_id.as_str());
    if highlight_nation.is_some() {
      window.draw(highlight_nation.unwrap().deref());
    }
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
