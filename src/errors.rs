use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct MapParseError;

impl error::Error for MapParseError {}
impl fmt::Display for MapParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "failed to parse map from geojson")
  }
}
