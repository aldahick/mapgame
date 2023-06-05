use geojson::Feature;

use crate::{errors::MapLoadError, geo_drawable::Bounds};

#[derive(Debug)]
pub struct Province {
  pub id: String,
  pub name: String,
}

impl Province {
  pub fn new(
    _feature: Feature,
    _bounds: &Bounds,
    _map_name: String,
  ) -> Result<Box<Province>, MapLoadError> {
    let province = Box::new(Province {
      id: String::default(),
      name: String::default(),
    });
    Ok(province)
  }
}
