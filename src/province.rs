use std::{collections::HashMap, error::Error, path::Path};

use geojson::{Feature, FeatureCollection, GeoJson};
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

impl Province {
  pub async fn load(
    config: &Config,
    nation_id: String,
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
      let province = Province::new(feature, &bounds)?;
      provinces.insert(province.geo_drawable.id.clone(), province);
    }
    println!(
      "Loaded {} provinces for nation {}",
      provinces.len(),
      nation_id
    );
    Ok(Some(provinces))
  }

  pub fn new(feature: Feature, bounds: &Bounds) -> Result<Box<Province>, Box<dyn Error>> {
    let geo_drawable = GeoDrawable::new(feature, bounds, "name", None)?;
    let mut province = Box::new(Province { geo_drawable });
    province.update_cached_vertices();
    Ok(province)
  }

  pub fn update_cached_vertices(&mut self) {
    self.geo_drawable.update_cached_vertices(Color::BLACK);
  }
}
