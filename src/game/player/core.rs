use std::str;

use bevy::{color::palettes::css, core_pipeline::tonemapping::DebandDither, prelude::*};
use bevy_rapier3d::prelude::*;

use super::{camera_controller::CameraController, movement::GroundSensor};

#[derive(Default, Component, Reflect)]
pub(super) struct Player {
  /// 力が加わる向きと速度(大きさ)
  pub direction: Vec3,
  /// 重力の掛かる方向
  /// 正の値だと下向きの力が掛かり
  /// 負の値だと上向きの力が掛かる(ジャンプ)
  pub vertical_accel: f32,
  /// 水平方向の移動速度
  pub horizontal_speed: f32,
  /// 垂直方向の移動速度
  pub vertical_speed: f32,
}

#[derive(Default, Component, Reflect)]
pub(super) struct Body;

pub(super) fn init_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn((
      Player {
        vertical_accel: 1.0,
        horizontal_speed: 8.0,
        vertical_speed: 18.0,
        ..default()
      },
      Name::new("Player"),
      Collider::capsule_y(1.0, 0.4),
      PbrBundle {
        mesh: meshes.add(Capsule3d::new(0.4, 2.0)),
        material: materials.add(Color::Srgba(css::LIGHT_CYAN)),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..default()
      },
      RigidBody::KinematicVelocityBased,
      KinematicCharacterController {
        up: Vec3::Y,
        offset: CharacterLength::Absolute(0.01),
        snap_to_ground: Some(CharacterLength::Absolute(0.8)),
        max_slope_climb_angle: 45_f32.to_radians(),
        min_slope_slide_angle: 30_f32.to_radians(),
        ..default()
      },
      GroundSensor::default(),
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

      parent.spawn((
        Body,
        PbrBundle {
          mesh: meshes.add(Cuboid::new(0.4, 0.4, 1.6)),
          material: materials.add(Color::Srgba(css::BEIGE)),
          transform: Transform::from_xyz(1.0, 1.0, -0.2).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.3,
            0.2,
            0.0,
          )),
          ..default()
        },
      ));
    });
}

pub(super) fn update_player(
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut player_query: Query<(&GroundSensor, &Handle<StandardMaterial>)>,
) {
  if let Ok((ground_sensor, material_handle)) = player_query.get_single_mut() {
    if let Some(material) = materials.get_mut(material_handle) {
      material.base_color = Color::Srgba(if ground_sensor.grounded {
        css::LIGHT_CYAN
      } else {
        css::ORANGE
      });
    }
  }
}