use std::{
  error,
  fmt::{self, Formatter},
};

#[derive(Debug, Clone)]
pub struct MapLoadError {
  pub reason: String,
}

impl error::Error for MapLoadError {}
impl fmt::Display for MapLoadError {
  fn fmt<'a>(&self, f: &mut Formatter<'a>) -> fmt::Result {
    write!(f, "failed to load map: {}", self.reason)
  }
}
