use bevy::prelude::*;

use super::{camera_controller::update_camera_controller, player::init_player};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_player)
      .add_systems(Update, update_camera_controller);
  }
}
