use bevy::prelude::*;

use crate::game::state::GameState;

use super::{
  camera_controller::update_camera_controller,
  core::{init_player, update_player, Body, Player},
  movement::{update_grounded, update_input, update_movement, GroundSensor, MovementInput},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MovementInput>()
      .register_type::<Player>()
      .register_type::<Body>()
      .register_type::<GroundSensor>()
      .add_systems(Startup, init_player)
      .add_systems(
        Update,
        (
          (
            update_movement,
            update_player,
            (update_grounded).before(update_movement),
          ),
          (
            update_camera_controller,
            update_input.before(update_movement),
          )
            .run_if(in_state(GameState::InGame)),
        ),
      );
  }
}
