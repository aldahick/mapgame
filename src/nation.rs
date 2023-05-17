use geojson::{Feature, Value};
use sfml::{
  graphics::{Color, Drawable, PrimitiveType, Rect, Vertex},
  system::Vector2f,
};

use crate::{
  errors::MapParseError,
  worldmap::{MAX_LATITUDE, MAX_LONGITUDE},
};

pub struct Nation {
  polygons: Vec<Vec<Vec<Vec<f64>>>>,
}

impl Nation {
  pub fn new(feature: Feature) -> Result<Nation, MapParseError> {
    let geometry = match feature.geometry.ok_or_else(|| MapParseError) {
      Ok(g) => g,
      Err(e) => return Err(e),
    };
    let mut polygons = Vec::new();
    if let Value::Polygon(polygon) = geometry.value {
      polygons.push(polygon);
    } else if let Value::MultiPolygon(multi) = geometry.value {
      polygons.extend(multi);
    }
    Ok(Nation { polygons })
  }

  fn to_vertex_groups(&self, color: Color, bounds: Rect<f64>) -> Vec<Vec<Vertex>> {
    let mut vertex_groups = Vec::new();
    let zero = Vector2f::new(0.0, 0.0);
    for polygon in &self.polygons {
      // see: https://stevage.github.io/geojson-spec/#section-3.1.6
      for linear_ring in polygon {
        let mut vertex_group = Vec::new();
        if let Some((_last, points)) = linear_ring.as_slice().split_last() {
          for point in points {
            let vector = Nation::to_vector(point, bounds);
            vertex_group.push(Vertex::new(vector, color, zero));
          }
        }
        vertex_groups.push(vertex_group);
      }
    }
    vertex_groups
  }

  fn to_vector(point: &Vec<f64>, bounds: Rect<f64>) -> Vector2f {
    let latitude = point[0];
    let longitude = point[1];
    let x = (latitude + MAX_LATITUDE) * bounds.width / (2.0 * MAX_LATITUDE);
    let y = bounds.height - ((longitude + MAX_LONGITUDE) * bounds.height / (2.0 * MAX_LONGITUDE));
    Vector2f::new(x as f32, y as f32)
  }
}

impl Drawable for Nation {
  fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
    &'a self,
    target: &mut dyn sfml::graphics::RenderTarget,
    states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
  ) {
    let viewport = target.viewport(target.view()).as_other::<f64>();
    let vertex_groups = self.to_vertex_groups(Color::BLACK, viewport);
    for vertices in vertex_groups {
      target.draw_primitives(vertices.as_slice(), PrimitiveType::LINE_STRIP, states);
    }
  }
}
