use sfml::{
  graphics::{RenderTarget, RenderWindow},
  system::Vector2f,
};
use tokio::fs::read_to_string;

use std::{collections::HashMap, error::Error, ops::Deref, path::Path};

use geojson::FeatureCollection;
use sfml::graphics::Rect;

use crate::{
  config::Config,
  errors::MapLoadError,
  geo_drawable::Bounds,
  nation::{Nation, Nations},
};

// simply too many nations. ideally this will be removed
pub const MIN_NATION_AREA: f32 = 0.25;

pub struct WorldMap {
  pub name: String,
  pub nations: Nations,
  pub highlighted_nation_id: Option<String>,
}

impl WorldMap {
  pub async fn new<'a>(config: &Config) -> Result<WorldMap, Box<dyn Error>> {
    let path_str = config.nations_path.as_str();
    let path = Path::new(path_str);
    let name = path
      .file_stem()
      .and_then(|n| Some(n.to_str().unwrap_or_default().to_string()))
      .ok_or_else(|| MapLoadError {
        reason: format!("path '{}' has invalid file name", path_str),
      })?;
    let nations = WorldMap::load_nations(config).await?;
    Ok(WorldMap {
      name,
      nations,
      highlighted_nation_id: None,
    })
  }

  pub fn render(&self, window: &mut RenderWindow) {
    let highlight_id = self.highlighted_nation_id.clone().unwrap_or_default();
    for (id, nation) in &self.nations {
      if id.as_str() != highlight_id {
        window.draw(nation.deref());
      }
    }
    let highlight_nation = self.nations.get(highlight_id.as_str());
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

  async fn load_nations(config: &Config) -> Result<Nations, Box<dyn Error>> {
    let geojson_str = read_to_string(&config.nations_path).await?;
    let features = WorldMap::parse_features(geojson_str)?;
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    let mut nations = HashMap::new();
    for feature in features {
      let nation = Nation::new(feature, &bounds, config).await?;
      let nation_id = nation.id().clone();
      if nation.area() > MIN_NATION_AREA {
        nations.insert(nation_id, nation);
      } else {
        println!(
          "nation {} is NOT big enough at {}",
          nation_id,
          nation.area()
        );
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
    self
      .highlighted_nation_id
      .clone()
      .and_then(|id| self.nations.get(id.as_str()))
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
