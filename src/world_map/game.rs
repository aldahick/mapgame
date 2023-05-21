use sfml::system::Vector2f;

use crate::nation::types::Nations;

use super::types::WorldMap;

impl WorldMap {
  /* Highlights the nation at `position` and unhighlights all others, returning the highlighted nation ID (if any) */
  pub fn set_highlighted_nation_at(nations: &mut Nations, position: Vector2f) -> Option<&String> {
    let mut highlighted_id = None;
    for (id, nation) in nations {
      nation.highlighted = nation.includes(position);
      if nation.highlighted {
        highlighted_id = Some(id);
      }
    }
    highlighted_id
  }

  /* Selects the highlighted nation and unselects the old one (if any) */
  pub fn set_selected_nation(
    nations: &mut Nations,
    old_selected_id_opt: Option<String>,
    new_selected_id: String,
  ) {
    if old_selected_id_opt.is_some() {
      let old_selected_id = old_selected_id_opt.unwrap();
      let old_nation = nations.get_mut(old_selected_id.as_str());
      if old_nation.is_some() {
        old_nation.unwrap().selected = false;
      }
    }
    let new_nation = nations.get_mut(new_selected_id.as_str());
    if new_nation.is_some() {
      new_nation.unwrap().selected = true;
    }
  }
}
