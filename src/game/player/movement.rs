use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player::Player;

pub(super) fn update_movement(
  key: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut query: Query<(
    &mut Player,
    &Transform,
    &mut KinematicCharacterController,
    Option<&KinematicCharacterControllerOutput>,
  )>,
) {
  if let Ok((mut player, player_transform, mut controller, controller_output)) =
    query.get_single_mut()
  {
    // TODO:directionをVec2,重力を別の変数にする
    // Vec3(x,y,z) Vec2(x,z)
    let mut horizontal_direction = Vec2::ZERO;

    if key.pressed(KeyCode::KeyW) {
      horizontal_direction += player_transform.forward().xz();
    }

    if key.pressed(KeyCode::KeyA) {
      horizontal_direction += player_transform.left().xz();
    }

    if key.pressed(KeyCode::KeyS) {
      horizontal_direction += player_transform.back().xz();
    }

    if key.pressed(KeyCode::KeyD) {
      horizontal_direction += player_transform.right().xz();
    }

    horizontal_direction = horizontal_direction.clamp_length(0.0, 1.0) * player.horizontal_speed;

    if let Some(controller_output) = controller_output {
      // 地面に付いて無いときは重力を加える
      if controller_output.grounded {
        player.gravity = 0.0;
        if key.pressed(KeyCode::Space) {
          player.gravity = -16.0;
        }
      } else {
        player.gravity += 4.0 * player.vertical_speed * time.delta_seconds();
      }

      player.direction.y -= player.gravity * 0.2;
    }

    player.direction = Vec3::from((
      horizontal_direction.x,
      player.direction.y,
      horizontal_direction.y,
    )) * time.delta_seconds();

    controller.translation = Some(player.direction);
  }
}
