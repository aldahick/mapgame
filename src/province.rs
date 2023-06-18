use std::{collections::HashMap, error::Error, path::Path};

use geojson::{Feature, FeatureCollection, GeoJson};
use sfml::graphics::Rect;
use tokio::fs::read_to_string;

use crate::{
  config::Config,
  geo_drawable::{Bounds, GeoDrawable},
};

#[derive(Debug)]
pub struct Province {
  pub id: String,
  pub name: String,
  pub geo_drawable: Box<GeoDrawable>,
}
pub type Provinces = HashMap<String, Box<Province>>;

impl Province {
  pub async fn load(config: &Config, id: String) -> Result<Option<Provinces>, Box<dyn Error>> {
    let path = Path::new(&config.provinces_dir).join(id + ".json");
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
      provinces.insert(province.id.clone(), province);
    }
    Ok(Some(provinces))
  }

  pub fn new(feature: Feature, bounds: &Bounds) -> Result<Box<Province>, Box<dyn Error>> {
    let geo_drawable = GeoDrawable::new(feature, bounds, "name", None)?;
    let province = Box::new(Province {
      id: String::default(),
      name: String::default(),
      geo_drawable,
    });
    Ok(province)
  }
}
