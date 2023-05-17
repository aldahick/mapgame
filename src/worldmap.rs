use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;
use geojson::FeatureCollection;
use std::{error, fs};

use crate::errors::MapParseError;
use crate::nation::Nation;

pub struct WorldMap {
  nations: Vec<Nation>,
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
    let nations = match WorldMap::to_nations(features) {
      Ok(n) => n,
      Err(e) => return Err(Box::new(e))
    };
    Ok(WorldMap {
      nations,
    })
  }

  fn to_nations(features: FeatureCollection) -> Result<Vec<Nation>, MapParseError> {
    let mut nations = Vec::new();
    for feature in features {
      let nation = match Nation::new(feature) {
        Ok(n) => n,
        Err(e) => return Err(e),
      };
      nations.push(nation);
    }
    Ok(nations)
  }

  pub fn render(&mut self, window: &mut RenderWindow) {
    for nation in &self.nations {
      window.draw(nation);
    }
  }
}
