use bevy::{input::mouse::MouseMotion, prelude::*};

use super::player::Player;

#[derive(Debug, Component)]
pub struct CameraController {
  pub sensitivity: f32,
}

pub fn update_camera_controller(
  mut mouse_motion: EventReader<MouseMotion>,
  mut player: Query<&mut Transform, With<Player>>,
  mut camera_controller: Query<(&mut CameraController, &mut Transform), Without<Player>>,
) {
  if let (Ok(mut player), Ok((camera_controller, mut camera_controller_transform))) =
    (player.get_single_mut(), camera_controller.get_single_mut())
  {
    for motion in mouse_motion.read() {
      let (_, pitch, _) = camera_controller_transform.rotation.to_euler(EulerRot::YXZ);
      // 左右
      player.rotate_y(-motion.delta.x * camera_controller.sensitivity);
      let pitch = pitch - motion.delta.y * camera_controller.sensitivity;
      // 上下
      camera_controller_transform.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, pitch, 0.0);
    }
  };
}
