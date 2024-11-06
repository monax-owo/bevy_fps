use core::f32;

use bevy::{input::mouse::MouseMotion, prelude::*};

use super::core::Player;

#[derive(Component, Reflect, Debug)]
pub(super) struct CameraController {
  /// カメラの感度
  pub(super) sensitivity: f32,
}

pub(super) fn update_camera_controller(
  mut mouse_motion: EventReader<MouseMotion>,
  mut player: Query<&mut Transform, With<Player>>,
  mut camera_controller: Query<(&mut CameraController, &mut Transform), Without<Player>>,
) {
  if let (Ok(mut player), Ok((camera_controller, mut camera_controller_transform))) =
    (player.get_single_mut(), camera_controller.get_single_mut())
  {
    for motion in mouse_motion.read() {
      // 左右
      player.rotate_y(-motion.delta.x * camera_controller.sensitivity);

      // 上下
      camera_controller_transform.rotation = Quat::from_rotation_x(
        (camera_controller_transform
          .rotation
          .to_euler(EulerRot::YXZ)
          .1
          - motion.delta.y * camera_controller.sensitivity)
          .clamp(-f32::consts::FRAC_PI_2, f32::consts::FRAC_PI_2),
      );
    }
  };
}
