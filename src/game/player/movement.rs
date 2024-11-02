use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player::Player;

#[derive(Default, Component, Reflect)]
pub(super) struct GroundSensor(bool);

pub(super) fn update_movement(
  key: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  rapier_context: Res<RapierContext>,
  mut player_query: Query<(&mut Player, &Transform, &mut KinematicCharacterController)>,
) {
  if let Ok((mut player, player_transform, mut controller)) = player_query.get_single_mut() {
    // Vec3(x,y,z) Vec2(x,z)
    let mut direction = Vec2::ZERO;

    if key.pressed(KeyCode::KeyW) {
      direction += player_transform.forward().xz();
    }

    if key.pressed(KeyCode::KeyA) {
      direction += player_transform.left().xz();
    }

    if key.pressed(KeyCode::KeyS) {
      direction += player_transform.back().xz();
    }

    if key.pressed(KeyCode::KeyD) {
      direction += player_transform.right().xz();
    }

    direction = direction.clamp_length(0.0, 1.0) * player.horizontal_speed;

    player.grounded = rapier_context
      .cast_ray(
        Vec3::new(
          player_transform.translation.x,
          player_transform.translation.y - 1.4,
          player_transform.translation.z,
        ),
        -Vec3::Y,
        0.02,
        true,
        QueryFilter::exclude_kinematic(),
      )
      .is_some();

    // 地面に付いて無いときは重力を加える
    if player.grounded {
      player.gravity = 0.0;
      if key.pressed(KeyCode::Space) {
        player.gravity = -64.0;
      }
    } else {
      player.gravity =
        (player.gravity + 9.8 * player.vertical_speed * time.delta_seconds()).clamp(-500.0, 500.0);
    }

    if player.gravity != 0.0 {
      player.direction.y -= player.gravity * 0.2;
    }

    player.direction =
      Vec3::from((direction.x, player.direction.y, direction.y)) * time.delta_seconds();

    // TODO:translationじゃなくてvelocityを使う？
    controller.translation = Some(player.direction);
  }
}

// TODO:接地判定を分ける
pub(super) fn _grounded(_ground_sensor_query: Query<(&GroundSensor, &Transform)>) {}
