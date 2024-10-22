use bevy::prelude::*;

use super::init_player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_player);
  }
}
