use geojson::Feature;

use crate::{
  errors::MapLoadError,
  geo_drawable::{Bounds, GeoDrawable},
};

#[derive(Debug)]
pub struct Province {
  pub id: String,
  pub name: String,
  pub geo_drawable: Box<GeoDrawable>,
}

impl Province {
  pub fn new(
    feature: Feature,
    bounds: &Bounds,
    map_name: String,
  ) -> Result<Box<Province>, MapLoadError> {
    let geo_drawable = GeoDrawable::new(feature, bounds, map_name, "name", None)?;
    let province = Box::new(Province {
      id: String::default(),
      name: String::default(),
      geo_drawable,
    });
    Ok(province)
  }
}
