use std::{collections::HashMap, error::Error, fs, path::Path};

use geojson::FeatureCollection;
use sfml::graphics::Rect;

use crate::{
  config::Config,
  errors::MapLoadError,
  nation::types::{Nation, Nations},
};

use super::types::{Bounds, WorldMap, MIN_NATION_AREA};

impl WorldMap {
  pub fn new<'a>(config: &Config) -> Result<WorldMap, Box<dyn Error>> {
    let path_str = config.nations_path.as_str();
    let path = Path::new(path_str);
    let name = path
      .file_stem()
      .and_then(|n| Some(n.to_str().unwrap_or_default().to_string()))
      .ok_or_else(|| MapLoadError {
        name: String::default(),
        reason: format!("path '{}' has invalid file name", path_str),
      })?;
    let nations = WorldMap::load_nations(config, &name)?;
    Ok(WorldMap {
      name: name.clone(),
      nations,
      highlighted_nation_id: None,
    })
  }

  fn parse_features(geojson_str: String) -> Result<FeatureCollection, geojson::Error> {
    let geojson = geojson_str.parse::<geojson::GeoJson>()?;
    geojson::FeatureCollection::try_from(geojson)
  }

  fn load_nations(config: &Config, map_name: &String) -> Result<Nations, Box<dyn Error>> {
    let geojson_str = fs::read_to_string(&config.nations_path)?;
    let features = WorldMap::parse_features(geojson_str)?;
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    let nations = WorldMap::to_nations(features, bounds, map_name)?;
    Ok(nations)
  }

  fn to_nations(
    features: FeatureCollection,
    bounds: Bounds,
    map_name: &String,
  ) -> Result<Nations, MapLoadError> {
    let mut nations = HashMap::new();
    for feature in features {
      let nation = Nation::new(feature, bounds, map_name.clone())?;
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
