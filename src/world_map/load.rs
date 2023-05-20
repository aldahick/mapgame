use std::{collections::HashMap, fs, path::Path};

use geojson::FeatureCollection;
use sfml::graphics::Rect;

use crate::{
  errors::MapLoadError,
  nation::types::{Nation, Nations},
};

use super::types::{Bounds, WorldMap, MIN_NATION_AREA};

impl WorldMap {
  pub fn new<'a>(path_str: String) -> Result<WorldMap, MapLoadError> {
    let path = Path::new(path_str.as_str());
    let name = path
      .file_stem()
      .and_then(|n| Some(n.to_str().unwrap_or_default().to_string()))
      .ok_or_else(|| MapLoadError {
        name: String::default(),
        reason: format!("path '{}' has invalid file name", path_str),
      })?;
    let geojson_str = fs::read_to_string(path).or_else(|e| {
      Err(MapLoadError {
        name: name.clone(),
        reason: e.kind().to_string(),
      })
    })?;
    let features = WorldMap::parse_features(geojson_str).or_else(|e| {
      Err(MapLoadError {
        name: name.clone(),
        reason: e.to_string(),
      })
    })?;
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    let nations = WorldMap::to_nations(features, bounds, name.clone())?;
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

  fn to_nations(
    features: FeatureCollection,
    bounds: Bounds,
    map_name: String,
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
