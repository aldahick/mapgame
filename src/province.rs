use std::{collections::HashMap, error::Error, path::Path};

use geojson::{Feature, FeatureCollection, GeoJson};
use serde::Deserialize;
use sfml::graphics::{Color, Rect};
use tokio::fs::read_to_string;

use crate::{
  config::Config,
  geo_drawable::{Bounds, GeoDrawable},
};

#[derive(Debug)]
pub struct Province {
  pub geo_drawable: Box<GeoDrawable>,
}
pub type Provinces = HashMap<String, Box<Province>>;

#[derive(Deserialize)]
pub struct ProvinceMapping {
  pub name: Option<String>,
  pub id: Option<String>,
}
pub type ProvinceMappings = HashMap<String, ProvinceMapping>;

const DEFAULT_NAME_PROPERTY: &str = "name";

impl Province {
  pub async fn load_mappings(
    config: &Config,
  ) -> Result<HashMap<String, ProvinceMapping>, Box<dyn Error>> {
    let json_str = read_to_string(&config.province_mappings_path).await?;
    Ok(serde_json::from_str(&json_str)?)
  }

  pub async fn load_nation(
    config: &Config,
    nation_id: String,
    mapping: Option<&ProvinceMapping>,
  ) -> Result<Option<Provinces>, Box<dyn Error>> {
    let path = Path::new(&config.provinces_dir).join(nation_id.clone() + ".json");
    if !path.exists() {
      return Ok(None);
    }
    let geojson_str = read_to_string(path).await?;
    let geojson = geojson_str.parse::<GeoJson>()?;
    let features = FeatureCollection::try_from(geojson)?;
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    let mut provinces = HashMap::new();
    for feature in features {
      let province = Province::new(feature, &bounds, mapping)?;
      provinces.insert(province.geo_drawable.id.clone(), province);
    }
    println!(
      "Loaded {} provinces for nation {}",
      provinces.len(),
      nation_id
    );
    Ok(Some(provinces))
  }

  pub fn new(
    feature: Feature,
    bounds: &Bounds,
    mapping: Option<&ProvinceMapping>,
  ) -> Result<Box<Province>, Box<dyn Error>> {
    let id_property = mapping.and_then(|m| m.id.as_ref().and_then(|i| Some(i.as_str())));
    let name_property = mapping
      .and_then(|m| m.name.clone())
      .unwrap_or_else(|| DEFAULT_NAME_PROPERTY.to_string());
    let geo_drawable = GeoDrawable::new(feature, bounds, name_property.as_str(), id_property)?;
    let mut province = Box::new(Province { geo_drawable });
    province.update_cached_vertices();
    Ok(province)
  }

  pub fn update_cached_vertices(&mut self) {
    self.geo_drawable.update_cached_vertices(Color::BLACK);
  }
}
