use sfml::system::Vector2f;

use crate::nation::types::Nations;

use super::types::WorldMap;

impl WorldMap {
  /* Highlights the nation at `position` and unhighlights all others */
  pub fn get_highlighted_nation_at(nations: &mut Nations, position: Vector2f) -> Option<String> {
    let mut highlighted_id = None;
    for (id, nation) in nations {
      if nation.includes(position) {
        highlighted_id = Some(id.to_string());
        nation.set_highlight(true);
      } else {
        nation.set_highlight(false);
      }
    }
    highlighted_id
  }
}
