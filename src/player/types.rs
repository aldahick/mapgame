pub struct Player {
  pub nation_id: Option<String>,
}

impl Player {
  pub fn new() -> Box<Player> {
    Box::new(Player { nation_id: None })
  }
}
