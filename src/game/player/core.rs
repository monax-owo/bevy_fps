use bevy::{
  color::palettes::css,
  core_pipeline::tonemapping::DebandDither,
  prelude::*,
  window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;
use inventory::Inventory;

use crate::game::{
  inventory::PlayerInventory,
  shooting::{weapons::TestGun, Shooter},
};

use super::{camera_controller::CameraController, movement::GroundSensor};

#[derive(Component, Reflect, Debug, Default)]
pub struct Player {
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

#[derive(Component, Reflect, Debug, Default)]
pub(super) struct Body;

// TODO:state側で管理する？
pub(super) fn on_enter(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
  if let Ok(window) = window_query.get_single_mut() {
    println!("enter");
    util_cursor_lock(window, true);
  }
}

pub(super) fn on_exit(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
  if let Ok(window) = window_query.get_single_mut() {
    util_cursor_lock(window, false);
  }
}

pub(super) fn util_cursor_lock(mut window: Mut<'_, Window>, val: bool) {
  window.cursor.grab_mode = if val {
    CursorGrabMode::Locked
  } else {
    CursorGrabMode::None
  };
  window.cursor.visible = !val;
}

// TODO:別のファイルに移す
pub(super) fn init_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let weapon = commands
    .spawn((
      Name::new("Weapon"),
      TransformBundle {
        local: Transform::from_xyz(1.0, -0.8, -1.0),
        ..default()
      },
      Shooter::default(),
      TestGun {
        bullet_speed: 140.0,
        ..default()
      },
    ))
    .id();

  let player = commands
    .spawn((
      Name::new("Player"),
      Player {
        vertical_accel: 1.0,
        horizontal_speed: 8.0,
        vertical_speed: 18.0,
        ..default()
      },
      PlayerInventory::new(Inventory::new(default(), 2), weapon),
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
    .id();

  let camera = commands
    .spawn((
      Name::new("Camera"),
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
    ))
    .id();

  let body = commands
    .spawn((
      Name::new("Body"),
      Body,
      PbrBundle {
        mesh: meshes.add(Cuboid::new(0.4, 0.4, 1.0)),
        material: materials.add(Color::Srgba(css::BEIGE)),
        transform: Transform::from_xyz(1.0, -0.8, -0.4).with_rotation(Quat::from_euler(
          EulerRot::XYZ,
          0.06,
          0.0,
          0.0,
        )),
        ..default()
      },
    ))
    .id();

  commands.entity(player).add_child(camera);
  commands.entity(camera).push_children(&[body, weapon]);
}

pub(super) fn update_grounded_color(
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
