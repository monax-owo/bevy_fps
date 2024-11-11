use bevy::prelude::*;

use crate::game::state::GameState;

use super::{
  camera_controller::{update_camera_controller, CameraController},
  init_player,
  input::{update_input, PlayerInput},
  movement::{update_grounded, update_movement, GroundSensor},
  on_enter, on_exit, update_grounded_color, Body, Player,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::InGame), on_enter)
      .add_systems(OnExit(GameState::InGame), on_exit)
      .add_systems(Startup, init_player)
      .add_systems(
        Update,
        (
          (
            update_movement,
            update_grounded_color,
            (update_grounded).before(update_movement),
          ),
          (
            update_camera_controller,
            update_input.before(update_movement),
          )
            .run_if(in_state(GameState::InGame)),
        ),
      )
      .init_resource::<PlayerInput>()
      .register_type::<Body>()
      .register_type::<CameraController>()
      .register_type::<GroundSensor>()
      .register_type::<Player>();
  }
}
