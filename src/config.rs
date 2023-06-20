use std::{collections::HashMap, error::Error, fs::read_dir, path::Path};

pub struct MapConfig {
  pub name: String,
  pub nations_path: Box<Path>,
  pub provinces_dir: Box<Path>,
  pub province_mappings_path: Box<Path>,
}
pub type MapConfigs = HashMap<String, Box<MapConfig>>;

const MAPS_DIR: &str = "./maps";
const NATIONS_FILE: &str = "nations.geojson";
const PROVINCE_MAPPINGS_FILE: &str = "provinces.json";
const PROVINCES_DIR: &str = "provinces";

pub fn get_available_maps() -> Result<MapConfigs, Box<dyn Error>> {
  let mut maps = HashMap::new();
  let entries = read_dir(MAPS_DIR)?;
  for entry in entries {
    let name = entry?.file_name().into_string().unwrap();
    let base_path = Path::new(MAPS_DIR).join(name.clone());
    let map = Box::new(MapConfig {
      name: name.clone(),
      nations_path: base_path.clone().join(NATIONS_FILE).into(),
      province_mappings_path: base_path.clone().join(PROVINCE_MAPPINGS_FILE).into(),
      provinces_dir: base_path.clone().join(PROVINCES_DIR).into(),
    });
    maps.insert(name, map);
  }
  Ok(maps)
}
