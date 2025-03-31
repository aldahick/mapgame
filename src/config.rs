use crate::errors::MapLoadError;
use serde::Deserialize;
use std::{collections::HashMap, env, error::Error, fs, path::Path};

pub struct Config {
  pub map: MapConfig,
  pub view: ViewConfig,
}

pub struct MapConfig {
  pub name: String,
  pub nations_path: Box<Path>,
  pub provinces_dir: Box<Path>,
  pub province_mappings_path: Box<Path>,
}
pub type MapConfigs = HashMap<String, MapConfig>;

pub struct ViewConfig {
  pub min_zoom: f32,
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
  let config = get_config_json()?;
  let mut maps = get_available_maps(&config.maps_dir)?;
  let map = maps.remove(&config.map_name).ok_or_else(|| MapLoadError {
    reason: format!(
      "failed to find map {} in dir {}",
      config.map_name, config.maps_dir
    ),
  })?;
  Ok(Config {
    map,
    view: ViewConfig {
      min_zoom: config.min_zoom,
    },
  })
}

#[derive(Deserialize)]
struct ConfigJson {
  map_name: String,
  maps_dir: String,
  min_zoom: f32,
}

fn get_config_json() -> Result<ConfigJson, Box<dyn Error>> {
  let current_dir = env::current_dir()?;
  let current_path = current_dir.as_path();
  let path = Path::join(current_path, "config.json");
  let exists = path.try_exists().unwrap_or(false);
  if !exists {
    let default_path = Path::join(current_path, "config.default.json");
    fs::copy(default_path, &path)?;
  }
  let json = fs::read_to_string(path)?;
  Ok(serde_json::from_str(&json)?)
}

const NATIONS_FILE: &str = "nations.geojson";
const PROVINCE_MAPPINGS_FILE: &str = "provinces.json";
const PROVINCES_DIR: &str = "provinces";

fn get_available_maps(maps_dir: &str) -> Result<MapConfigs, Box<dyn Error>> {
  let mut maps = HashMap::new();
  let entries = fs::read_dir(maps_dir)?;
  for entry in entries {
    let name = entry?.file_name().into_string().or_else(|s| {
      Err(MapLoadError {
        reason: format!("failed to convert map file name to string: {:?}", s),
      })
    })?;
    let base_path = Path::new(maps_dir).join(&name);
    let map = MapConfig {
      name: name.clone(),
      nations_path: base_path.join(NATIONS_FILE).into(),
      province_mappings_path: base_path.join(PROVINCE_MAPPINGS_FILE).into(),
      provinces_dir: base_path.join(PROVINCES_DIR).into(),
    };
    maps.insert(name, map);
  }
  Ok(maps)
}
