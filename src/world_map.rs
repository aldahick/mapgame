use geojson::FeatureCollection;
use sfml::graphics::Rect;
use sfml::{
  graphics::{RenderTarget, RenderWindow},
  system::Vector2f,
};
use std::{collections::HashMap, error::Error, fs::read_to_string, ops::Deref};

use crate::{
  config::MapConfig,
  errors::MapLoadError,
  geo_drawable::Bounds,
  nation::{Nation, Nations},
  province::Province,
};

// simply too many nations. this will be removed
pub const MIN_NATION_AREA: f32 = 0.25;

pub struct WorldMap {
  nations: Nations,
  highlighted_nation_id: Option<String>,
  // 0,0 is the middle of the window, so this can be negative
  _view_center: Vector2f,
  // minimum 1, as this is the scale factor by which to zoom the map.
  pub zoom: f32,
}

impl WorldMap {
  pub fn new<'a>(config: &MapConfig) -> Result<WorldMap, Box<dyn Error>> {
    let nations = WorldMap::load_nations(config)?;
    Ok(WorldMap {
      nations,
      highlighted_nation_id: None,
      _view_center: Vector2f::new(0.0, 0.0),
      zoom: 1.0,
    })
  }

  fn parse_features(geojson_str: String) -> Result<FeatureCollection, geojson::Error> {
    let geojson = geojson_str.parse::<geojson::GeoJson>()?;
    geojson::FeatureCollection::try_from(geojson)
  }

  fn load_nations(config: &MapConfig) -> Result<Nations, Box<dyn Error>> {
    if !config.nations_path.exists() {
      return Err(Box::new(MapLoadError {
        reason: format!(
          "No nations JSON found at {:?}",
          config.nations_path.to_str()
        ),
      }));
    }
    let geojson_str = read_to_string(&config.nations_path)?;
    let features = WorldMap::parse_features(geojson_str)?;
    let province_mappings = Province::load_mappings(config)?;
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    let mut nations = HashMap::new();
    for feature in features {
      let nation = Nation::new(feature, &bounds, config, &province_mappings)?;
      let nation_id = nation.id().clone();
      if nation.area() > MIN_NATION_AREA {
        nations.insert(nation_id, nation);
      }
    }
    Ok(nations)
  }

  pub fn render(&self, window: &mut RenderWindow) {
    for nation in self.nations.values() {
      window.draw(nation.deref());
    }
    self.highlighted_nation_id.clone().and_then(|nation_id| {
      self.nations.get(&nation_id).and_then(|nation| {
        window.draw(nation.deref());
        Some(())
      })
    });
  }

  pub fn on_resize(&mut self, bounds: &Bounds) {
    for (_id, nation) in self.nations.iter_mut() {
      nation.on_resize(&bounds);
    }
  }

  /* Highlights the nation at `position` and unhighlights all others, returning the highlighted nation ID (if any) */
  pub fn set_highlighted_nation_at(&mut self, position: Vector2f) -> &Option<String> {
    for (id, nation) in self.nations.iter_mut() {
      if nation.includes(position) {
        nation.set_highlighted(true);
        self.highlighted_nation_id = Some(id.clone());
      } else if nation.is_highlighted() {
        nation.set_highlighted(false);
      }
    }
    &self.highlighted_nation_id
  }

  pub fn get_highlighted_nation(&self) -> Option<&Box<Nation>> {
    let highlighted_id = self.highlighted_nation_id.clone()?;
    self.nations.get(highlighted_id.as_str())
  }

  /* Selects the highlighted nation and unselects the old one (if any) */
  pub fn set_selected_nation(
    &mut self,
    old_selected_id_opt: Option<&String>,
    new_selected_id: &String,
  ) {
    old_selected_id_opt.and_then(|old_selected_id| {
      self.get_nation_mut(old_selected_id).and_then(|old_nation| {
        old_nation.set_selected(false);
        Some(())
      })
    });
    self.get_nation_mut(new_selected_id).and_then(|new_nation| {
      new_nation.set_selected(true);
      Some(())
    });
  }

  pub fn set_selected_nation_at(&mut self, position: Vector2f) -> Option<String> {
    for (id, nation) in self.nations.iter_mut() {
      if nation.includes(position) {
        nation.set_selected(true);
        return Some(id.clone());
      }
    }
    None
  }

  fn get_nation_mut(&mut self, id: &String) -> Option<&mut Box<Nation>> {
    self.nations.get_mut(id)
  }
}
