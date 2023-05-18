use sfml::system::Vector2f;

use super::types::WorldMap;

impl WorldMap {
  pub fn set_highlight(&mut self, position: Vector2f) {
    let target_id_opt = self.find_nation_id_at(position);
    self.highlighted_nation_id = target_id_opt.clone();
    let target_id = target_id_opt.unwrap_or_default();
    for (id, nation) in self.nations.iter_mut() {
      nation.set_highlight(target_id == id.as_str());
    }
  }
}
