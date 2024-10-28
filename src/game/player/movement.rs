use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player::Player;

// TODO:プレイヤーが奈落に落ちないのを治す
pub(super) fn update_movement(
  key: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut query: Query<(
    &Player,
    &Transform,
    &mut KinematicCharacterController,
    Option<&KinematicCharacterControllerOutput>,
  )>,
) {
  if let Ok((player, player_transform, mut controller, controller_output)) = query.get_single_mut()
  {
    // TODO:directionをVec2,重力を別の変数にする
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

    direction = direction.clamp_length(0.0, 1.0);

    if key.pressed(KeyCode::Space) {
      // TODO
      // direction += *player_transform.forward();
    }

    if let Some(controller_output) = controller_output {
      if !controller_output.grounded {
        direction.y -= player.gravity;
      }
    }

    controller.translation =
      Some(direction.normalize_or_zero() * player.speed * time.delta_seconds());
  }
}
