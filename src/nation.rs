use geojson::{Feature, Value};
use sfml::{
  graphics::{Color, Drawable, Vertex, PrimitiveType},
  system::Vector2f,
};

use crate::errors::MapParseError;

pub struct Nation {
  vertex_groups: Vec<Vec<Vertex>>,
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
    let vertex_groups = Nation::to_vertex_groups(polygons, Color::BLACK);
    Ok(Nation { vertex_groups })
  }

  fn to_vertex_groups(polygons: Vec<Vec<Vec<Vec<f64>>>>, color: Color) -> Vec<Vec<Vertex>> {
    let mut vertex_groups = Vec::new();
    let zero = Vector2f::new(0.0, 0.0);
    for polygon in polygons {
      // see: https://stevage.github.io/geojson-spec/#section-3.1.6
      for linear_ring in polygon {
        let mut vertex_group = Vec::new();
        if let Some((_last, points)) = linear_ring.as_slice().split_last() {
          for point in points {
            vertex_group.push(Vertex::new(Vector2f::new(point[0] as f32, point[1] as f32), color, zero));
          }
        }
        vertex_groups.push(vertex_group);
      }
    }
    vertex_groups
  }
}

impl Drawable for Nation {
  fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
    &'a self,
    target: &mut dyn sfml::graphics::RenderTarget,
    states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
  ) {
    for vertices in &self.vertex_groups {
      target.draw_primitives(vertices.as_slice(), PrimitiveType::LINE_STRIP, states);
    }
  }
}
