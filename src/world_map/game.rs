use sfml::system::Vector2f;

use crate::nation::types::Nation;

use super::types::WorldMap;

impl WorldMap {
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
