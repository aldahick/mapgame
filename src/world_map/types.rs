use sfml::{graphics::Rect, system::Vector2f};

use crate::nation::types::Nations;

pub const MAX_LATITUDE: f32 = 180.0;
pub const MAX_LONGITUDE: f32 = 90.0;

// simply too many nations. ideally this will be removed
pub const MIN_NATION_AREA: f32 = 0.25;

pub type Bounds = Rect<f32>;
pub type GeoPolygons = Vec<Vec<Vec<Vec<f64>>>>;
pub type VectorPolygons = Vec<Vec<Vector2f>>;

pub struct WorldMap {
  pub nations: Nations,
  pub highlighted_nation_id: Option<String>,
}
