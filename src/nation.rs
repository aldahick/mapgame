use std::{collections::HashMap, error::Error};

use geojson::Feature;
use sfml::{
  graphics::{Color, Drawable},
  system::Vector2f,
};

use crate::{
  config::MapConfig,
  geo_drawable::{Bounds, GeoDrawable},
  province::{Province, ProvinceMappings, Provinces},
};

#[derive(Debug)]
pub struct Nation {
  pub highlighted: bool,
  pub selected: bool,
  pub geo_drawable: Box<GeoDrawable>,
  pub provinces: Option<Provinces>,
}
pub type Nations = HashMap<String, Box<Nation>>;

impl Nation {
  pub async fn new(
    feature: Feature,
    bounds: &Bounds,
    config: &MapConfig,
    province_mappings: &ProvinceMappings,
  ) -> Result<Box<Nation>, Box<dyn Error>> {
    let geo_drawable = GeoDrawable::new(feature, bounds, "ADMIN", Some("ISO_A3"))?;
    let nation_id = geo_drawable.id.clone();
    let province_mapping = province_mappings.get(&nation_id);
    let provinces = Province::load_nation(config, nation_id, province_mapping).await?;
    let mut nation = Box::new(Nation {
      geo_drawable,
      highlighted: false,
      selected: false,
      provinces,
    });
    nation.update_cached_vertices();
    Ok(nation)
  }

  pub fn is_highlighted(&self) -> bool {
    self.highlighted
  }

  pub fn is_selected(&self) -> bool {
    self.selected
  }

  pub fn set_highlighted(&mut self, value: bool) {
    self.highlighted = value;
    self.update_cached_vertices();
  }

  pub fn set_selected(&mut self, value: bool) {
    self.selected = value;
    self.update_cached_vertices();
  }

  pub fn area(&self) -> f32 {
    self.geo_drawable.vector_total_area
  }

  pub fn id(&self) -> &String {
    &self.geo_drawable.id
  }

  pub fn includes(&self, position: Vector2f) -> bool {
    self.geo_drawable.includes(position)
  }

  pub fn on_resize(&mut self, bounds: &Bounds) {
    self.geo_drawable.on_resize(bounds);
    self.update_cached_vertices();
    if self.provinces.is_some() {
      for (_id, province) in self.provinces.as_mut().unwrap() {
        province.geo_drawable.on_resize(bounds);
        province.update_cached_vertices();
      }
    }
  }

  pub fn update_cached_vertices(&mut self) {
    let color = if self.is_selected() {
      Color::BLUE
    } else if self.is_highlighted() {
      Color::GREEN
    } else {
      Color::BLACK
    };
    self.geo_drawable.update_cached_vertices(color);
  }
}

impl Drawable for Nation {
  fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
    &'a self,
    target: &mut dyn sfml::graphics::RenderTarget,
    states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
  ) {
    if self.provinces.is_some() {
      for (_id, province) in self.provinces.as_ref().unwrap() {
        province.geo_drawable.draw(target, states);
      }
    }
    self.geo_drawable.draw(target, states);
  }
}
