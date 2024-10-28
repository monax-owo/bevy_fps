use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player::Player;

// TODO:プレイヤーが奈落に落ちないのを治す
pub(super) fn update_movement(
  key: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut query: Query<(&Player, &Transform, &mut KinematicCharacterController)>,
) {
  if let Ok((player, player_transform, mut kinematic_controller)) = query.get_single_mut() {
    let mut direction = Vec3::ZERO;

    if key.pressed(KeyCode::KeyW) {
      direction += *player_transform.forward();
    }

    if key.pressed(KeyCode::KeyA) {
      direction += *player_transform.left();
    }

    if key.pressed(KeyCode::KeyS) {
      direction += *player_transform.back();
    }

    if key.pressed(KeyCode::KeyD) {
      direction += *player_transform.right();
    }

    if key.pressed(KeyCode::Space) {
      // TODO
      // direction += *player_transform.forward();
    }

    kinematic_controller.translation =
      Some(direction.clamp_length(0.0, 1.0).normalize() * player.speed * time.delta_seconds());
  }
}
