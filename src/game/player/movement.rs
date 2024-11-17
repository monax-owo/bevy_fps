use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{input::PlayerInput, Player};

#[derive(Component, Reflect)]
pub(super) struct GroundSensor {
  /// 接地しているか
  pub grounded: bool,
  /// time-of-impact
  pub toi: f32,
}

impl Default for GroundSensor {
  fn default() -> Self {
    Self {
      grounded: Default::default(),
      // TODO:調整する
      toi: 0.16,
    }
  }
}

// ユーザーからの入力を反映する
pub(super) fn update_movement_input(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  key: Res<PlayerInput>,
  time: Res<Time>,
  mut player_query: Query<(&mut Player, &Transform, &GroundSensor)>,
) {
  // const JUMP_HEIGHT: f32 = -80.0;

  // if let Ok((mut player, player_transform, ground_sensor)) = player_query.get_single_mut() {
  //   if keyboard_input.pressed(key.forward) {
  //     player.test.x += 1.0;
  //   }

  //   if keyboard_input.pressed(key.left) {
  //     player.test.z += -1.0;
  //   }

  //   if keyboard_input.pressed(key.back) {
  //     player.test.x += -1.0;
  //   }

  //   if keyboard_input.pressed(key.right) {
  //     player.test.z += 1.0;
  //   }

  //   // TODO:プレイヤーが止まったら歩きの速度にする
  //   if keyboard_input.pressed(key.dash) {
  //     player.horizontal_speed = 20.0;
  //   }

  //   // 地面に付いて無いときは重力を加える
  //   if ground_sensor.grounded {
  //     // jump
  //     if keyboard_input.pressed(key.jump) {
  //       player.vertical_accel += JUMP_HEIGHT;
  //     }
  //   }
  // }
}

// Playerのプロパティを使用してエンティティを移動させる
pub(super) fn update_movement(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  key: Res<PlayerInput>,
  time: Res<Time>,
  mut player_query: Query<(
    &mut Player,
    &Transform,
    &mut KinematicCharacterController,
    &GroundSensor,
  )>,
) {
  const GRAVITY: f32 = 9.8;
  const JUMP_HEIGHT: f32 = -80.0;

  if let Ok((mut player, player_transform, mut controller, ground_sensor)) =
    player_query.get_single_mut()
  {
    if keyboard_input.pressed(key.forward) {
      player.test.x += 1.0;
    }

    if keyboard_input.pressed(key.left) {
      player.test.z += -1.0;
    }

    if keyboard_input.pressed(key.back) {
      player.test.x += -1.0;
    }

    if keyboard_input.pressed(key.right) {
      player.test.z += 1.0;
    }

    // TODO:プレイヤーが止まったら歩きの速度にする
    if keyboard_input.pressed(key.dash) {
      player.horizontal_speed = 20.0;
    }

    player.test =
      player.test.x * player_transform.forward() + player.test.z * player_transform.right();

    // 地面に付いて無いときは重力を加える
    if ground_sensor.grounded {
      player.vertical_accel = (player.vertical_accel
        - player.vertical_speed * 2.2 * time.delta_seconds())
      .clamp(9.8, 20.0);

      // jump
      if keyboard_input.pressed(key.jump) {
        player.vertical_accel += JUMP_HEIGHT;
      }
    } else {
      player.vertical_accel = (player.vertical_accel
        + GRAVITY * player.vertical_speed * time.delta_seconds())
      .clamp(-500.0, 500.0);
    }

    player.direction.y -= player.vertical_accel * 0.2;

    player.direction =
      (player.test * player.horizontal_speed).with_y(player.direction.y) * time.delta_seconds();

    controller.translation = Some(player.direction);
    player.test = Vec3::ZERO;
  }
}

pub(super) fn update_grounded(
  rapier_context: Res<RapierContext>,
  mut ground_sensor_query: Query<(&mut GroundSensor, &Transform)>,
) {
  // const HALF_HEIGHT: f32 = 0.2;
  // const RADIUS: f32 = 0.16;

  // ray castでも良さそう？->ray castにした
  // todo:おかしかったらshape castに戻す
  for (mut ground_sensor, transform) in ground_sensor_query.iter_mut() {
    ground_sensor.grounded = rapier_context
      .cast_ray(
        transform.translation.with_y(transform.translation.y - 1.4),
        -Vec3::Y,
        ground_sensor.toi,
        true,
        QueryFilter::exclude_kinematic(),
      )
      .is_some();
  }
}
