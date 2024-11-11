use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{input::PlayerInput, Player};

#[derive(Component, Reflect)]
pub(super) struct GroundSensor {
  /// 接地しているか
  pub grounded: bool,
  pub toi: f32,
}

impl Default for GroundSensor {
  fn default() -> Self {
    Self {
      grounded: Default::default(),
      // TODO:調整する
      toi: 0.06,
    }
  }
}

pub(super) fn update_movement(
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
    let mut direction = Vec3::ZERO;

    if key.forward {
      direction.x += 1.0;
    }

    if key.left {
      direction.z += -1.0;
    }

    if key.back {
      direction.x += -1.0;
    }

    if key.right {
      direction.z += 1.0;
    }

    // TODO:プレイヤーが止まったら歩きの速度にする
    if key.dash {
      player.horizontal_speed = 20.0;
    }

    direction = direction.x * player_transform.forward() + direction.z * player_transform.right();

    // 地面に付いて無いときは重力を加える
    if ground_sensor.grounded {
      player.vertical_accel = (player.vertical_accel
        - player.vertical_speed * 2.2 * time.delta_seconds())
      .clamp(9.8, 20.0);

      // jump
      if key.jump {
        player.vertical_accel += JUMP_HEIGHT;
      }
    } else {
      player.vertical_accel = (player.vertical_accel
        + GRAVITY * player.vertical_speed * time.delta_seconds())
      .clamp(-500.0, 500.0);
    }

    player.direction.y -= player.vertical_accel * 0.2;

    player.direction =
      (direction * player.horizontal_speed).with_y(player.direction.y) * time.delta_seconds();

    controller.translation = Some(player.direction);
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
    // ground_sensor.grounded = rapier_context
    //   .cast_shape(
    //     transform
    //       .translation
    //       .with_y(transform.translation.y - 1.4 + HALF_HEIGHT),
    //     Quat::default(),
    //     -Vec3::Y,
    //     &Collider::cylinder(HALF_HEIGHT, RADIUS),
    //     ShapeCastOptions {
    //       max_time_of_impact: 0.06,
    //       ..default()
    //     },
    //     QueryFilter::exclude_kinematic(),
    //   )
    //   .is_some();

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
