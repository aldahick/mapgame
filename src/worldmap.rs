use sfml::graphics::Rect;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;
use geojson::FeatureCollection;
use sfml::system::Vector2f;
use std::collections::HashMap;
use std::{error, fs};

use crate::errors::MapParseError;
use crate::nation::Nation;
use crate::nation::Nations;

pub const MAX_LATITUDE: f32 = 180.0;
pub const MAX_LONGITUDE: f32 = 90.0;

// simply too many nations. ideally this will be removed
const MIN_NATION_AREA: f32 = 0.25;

pub type Bounds = Rect<f32>;
pub type GeoPolygons = Vec<Vec<Vec<Vec<f64>>>>;
pub type VectorPolygons = Vec<Vec<Vector2f>>;

pub struct WorldMap {
  pub nations: Nations,
  highlighted_nation_id: Option<String>,
}

impl WorldMap {
  pub fn new(filename: String) -> Result<WorldMap, Box<dyn error::Error>> {
    let geojson_str = match fs::read_to_string(filename) {
      Ok(g) => g,
      Err(e) => return Err(Box::new(e))
    };
    let geojson = match geojson_str.parse::<geojson::GeoJson>() {
      Ok(g) => g,
      Err(e) => return Err(Box::new(e))
    };
    let features = match geojson::FeatureCollection::try_from(geojson) {
      Ok(f) => f,
      Err(e) => return Err(Box::new(e))
    };
    let nations = match WorldMap::to_nations(features, Rect::new(0.0, 0.0, 100.0, 100.0)) {
      Ok(n) => n,
      Err(e) => return Err(Box::new(e))
    };
    Ok(WorldMap {
      nations,
      highlighted_nation_id: None,
    })
  }

  fn to_nations(features: FeatureCollection, bounds: Bounds) -> Result<Nations, MapParseError> {
    let mut nations = HashMap::new();
    for feature in features {
      let nation = match Nation::new(feature, bounds) {
        Ok(n) => n,
        Err(e) => return Err(e),
      };
      if nation.area() > MIN_NATION_AREA {
        nations.insert(nation.id.clone(), nation);
      } else {
        println!("nation {} is NOT big enough at {}", nation.id, nation.area());
      }
    }
    Ok(nations)
  }

  pub fn find_nation_id_at(&self, position: Vector2f) -> Option<String> {
    for (id, nation) in &self.nations {
      if nation.includes(position) {
        return Some(id.to_string());
      }
    }
    None
  }

  pub fn set_highlight(&mut self, position: Vector2f) {
    let target_id_opt = self.find_nation_id_at(position);
    self.highlighted_nation_id = target_id_opt.clone();
    let target_id = target_id_opt.unwrap_or_default();
    for (id, nation) in self.nations.iter_mut() {
      nation.set_highlight(target_id == id.as_str());
    }
  }

  pub fn render(&self, window: &mut RenderWindow) {
    let highlight_id = self.highlighted_nation_id.clone().unwrap_or_default();
    for (id, nation) in &self.nations {
      if id.as_str() != highlight_id {
        window.draw(nation.as_ref());
      }
    }
    let highlight_nation = self.nations.get(highlight_id.as_str());
    if highlight_nation.is_some() {
      window.draw(highlight_nation.unwrap().as_ref());
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
}
