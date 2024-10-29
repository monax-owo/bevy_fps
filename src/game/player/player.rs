use bevy::{core_pipeline::tonemapping::DebandDither, prelude::*};
use bevy_rapier3d::prelude::*;

use super::camera_controller::CameraController;

#[derive(Default, Component, Reflect)]
pub(super) struct Player {
  /// 力が加わる向きと速度(大きさ)
  pub direction: Vec3,
  /// 重力の掛かる方向
  /// 正の値だと下向きの力が掛かり
  /// 負の値だと上向きの力が掛かる(ジャンプ)
  pub gravity: f32,
  /// 水平方向の移動速度
  pub horizontal_speed: f32,
  /// 垂直方向の移動速度
  pub vertical_speed: f32,
}

pub(super) fn init_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn((
      Player {
        direction: Vec3::ZERO,
        gravity: 1.0,
        horizontal_speed: 8.0,
        vertical_speed: 18.0,
      },
      Collider::cuboid(0.6, 1.4, 0.6),
      PbrBundle {
        mesh: meshes.add(Cuboid::new(1.2, 2.8, 1.2)),
        material: materials.add(Color::srgb_u8(0, 255, 255)),
        transform: Transform::from_xyz(0.0, 1.4, 0.0),
        ..default()
      },
      RigidBody::KinematicPositionBased,
      KinematicCharacterController {
        up: Vec3::Y,
        offset: CharacterLength::Absolute(0.001),
        snap_to_ground: Some(CharacterLength::Absolute(0.5)),
        ..default()
      },
    ))
    .with_children(|parent| {
      parent.spawn((
        Camera3dBundle {
          deband_dither: DebandDither::Disabled,
          camera: Camera {
            hdr: true,
            order: 1,
            ..default()
          },
          projection: Projection::Perspective(PerspectiveProjection {
            fov: 90.0,
            ..default()
          }),
          transform: Transform::from_xyz(0.0, 2.0, 0.0),
          ..default()
        },
        CameraController { sensitivity: 0.001 },
      ));
    });
}
