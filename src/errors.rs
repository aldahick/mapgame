use std::{
  error,
  fmt::{self, Formatter},
};

#[derive(Debug, Clone)]
pub struct MapParseError;

impl error::Error for MapParseError {}
impl fmt::Display for MapParseError {
  fn fmt<'a>(&self, f: &mut Formatter<'a>) -> fmt::Result {
    write!(f, "failed to parse map from geojson")
  }
}
