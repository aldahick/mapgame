use sfml::{
  graphics::{RenderTarget, RenderWindow},
  system::Vector2f,
};

use std::{collections::HashMap, error::Error, fs::read_to_string, ops::Deref};

use geojson::FeatureCollection;
use sfml::graphics::Rect;

use crate::{
  config::MapConfig,
  errors::MapLoadError,
  geo_drawable::Bounds,
  nation::{Nation, Nations},
  province::Province,
};

// simply too many nations. ideally this will be removed
pub const MIN_NATION_AREA: f32 = 0.25;

pub struct WorldMap {
  pub name: String,
  pub nations: Nations,
  pub highlighted_nation_id: Option<String>,
}

impl WorldMap {
  pub fn new<'a>(config: &MapConfig) -> Result<WorldMap, Box<dyn Error>> {
    let nations = WorldMap::load_nations(config)?;
    Ok(WorldMap {
      name: config.name.clone(),
      nations,
      highlighted_nation_id: None,
    })
  }

  pub fn render(&self, window: &mut RenderWindow) {
    let highlight_id = self.highlighted_nation_id.clone().unwrap_or_default();
    let mut highlight_nation: Option<&Box<Nation>> = None;
    for (id, nation) in &self.nations {
      if id.as_str() == highlight_id {
        highlight_nation = Some(nation);
      } else {
        window.draw(nation.deref());
      }
    }
    if highlight_nation.is_some() {
      window.draw(highlight_nation.unwrap().deref());
    }
  }

  pub fn on_resize(&mut self, bounds: &Bounds) {
    for (_id, nation) in self.nations.iter_mut() {
      nation.on_resize(&bounds);
    }
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

  /* Highlights the nation at `position` and unhighlights all others, returning the highlighted nation ID (if any) */
  pub fn set_highlighted_nation_at(&mut self, position: Vector2f) -> Option<String> {
    let mut highlighted_id = None;
    for (id, nation) in self.nations.iter_mut() {
      if nation.includes(position) {
        nation.set_highlighted(true);
        highlighted_id = Some(id.clone());
      } else if nation.is_highlighted() {
        nation.set_highlighted(false);
      }
    }
    highlighted_id
  }

  pub fn get_highlighted_nation(&self) -> Option<&Box<Nation>> {
    let highlighted_id = self.highlighted_nation_id.clone()?;
    self.nations.get(highlighted_id.as_str())
  }

  /* Selects the highlighted nation and unselects the old one (if any) */
  pub fn set_selected_nation(
    &mut self,
    old_selected_id_opt: Option<String>,
    new_selected_id: String,
  ) {
    if old_selected_id_opt.is_some() {
      let old_selected_id = old_selected_id_opt.unwrap();
      let old_nation = self.nations.get_mut(old_selected_id.as_str());
      if old_nation.is_some() {
        old_nation.unwrap().set_selected(false);
      }
    }
    let new_nation = self.nations.get_mut(new_selected_id.as_str());
    if new_nation.is_some() {
      new_nation.unwrap().set_selected(true);
    }
  }
}
