use std::{collections::HashMap, error, fs};

use geojson::FeatureCollection;
use sfml::graphics::Rect;

use crate::{
  errors::MapParseError,
  nation::types::{Nation, Nations},
};

use super::types::{Bounds, WorldMap, MIN_NATION_AREA};

impl WorldMap {
  pub fn new<'a>(filename: String) -> Result<WorldMap, Box<dyn error::Error>> {
    let geojson_str = match fs::read_to_string(filename) {
      Ok(g) => g,
      Err(e) => return Err(Box::new(e)),
    };
    let geojson = match geojson_str.parse::<geojson::GeoJson>() {
      Ok(g) => g,
      Err(e) => return Err(Box::new(e)),
    };
    let features = match geojson::FeatureCollection::try_from(geojson) {
      Ok(f) => f,
      Err(e) => return Err(Box::new(e)),
    };
    let nations = match WorldMap::to_nations(features, Rect::new(0.0, 0.0, 100.0, 100.0)) {
      Ok(n) => n,
      Err(e) => return Err(Box::new(e)),
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
        println!(
          "nation {} is NOT big enough at {}",
          nation.id,
          nation.area()
        );
      }
    }
    Ok(nations)
  }
}
