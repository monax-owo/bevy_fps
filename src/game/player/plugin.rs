use bevy::prelude::*;

use super::{
  camera_controller::update_camera_controller,
  movement::update_movement,
  player::{init_player, Player},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Player>()
      .add_systems(Startup, init_player)
      .add_systems(Update, (update_camera_controller, update_movement));
  }
}
