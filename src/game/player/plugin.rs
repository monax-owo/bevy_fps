use bevy::prelude::*;

use super::{
  camera_controller::update_camera_controller,
  movement::{update_grounded, update_movement, GroundSensor},
  player::{init_player, update_player, Body, Player},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Player>()
      .register_type::<Body>()
      .register_type::<GroundSensor>()
      .add_systems(Startup, init_player)
      .add_systems(
        Update,
        (
          update_camera_controller,
          update_movement,
          update_player,
          update_grounded,
        ),
      );
  }
}
