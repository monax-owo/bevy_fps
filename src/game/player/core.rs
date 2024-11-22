use bevy::{color::palettes::css, core_pipeline::tonemapping::DebandDither, prelude::*};
use bevy_rapier3d::prelude::*;
use inventory::Inventory;

use crate::game::shooting::{weapons::ExampleGun, Shooter};

use super::{camera_controller::CameraController, movement::GroundSensor};

pub const PLAYER_HALF_HEIGHT: f32 = 1.0;
pub const PLAYER_RADIUS: f32 = 0.4;
pub const PLAYER_HEIGHT: f32 = PLAYER_HALF_HEIGHT + PLAYER_RADIUS;
pub const PLAYER_OFFSET: f32 = 0.01;

#[derive(Component, Reflect, Debug)]
pub struct Player {
  /// 力が加わる向きと速度(大きさ)
  pub direction: Vec3,
  /// 正の値だと下向きの力が掛かり
  /// 負の値だと上向きの力が掛かる(ジャンプ)
  pub vertical_accel: f32,
  /// 水平方向の移動速度
  pub horizontal_speed: f32,
  /// 垂直方向の移動速度
  pub vertical_speed: f32,
  /// 連続でジャンプできる回数(ダブルジャンプをさせたいなら2)
  pub jump_max_count: u32,
  /// 連続でジャンプする際のクールタイム
  pub jump_cool_time: f32,
}

impl Default for Player {
  fn default() -> Self {
    Self {
      direction: Default::default(),
      vertical_accel: Default::default(),
      horizontal_speed: 1.0,
      vertical_speed: 1.0,
      jump_max_count: 1,
      jump_cool_time: Default::default(),
    }
  }
}

#[derive(Component, Reflect, Debug, Default)]
pub(super) struct Body;

pub(super) fn init_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  // TODO: フローチャートにする
  // (`Player`,`Inventory`,`PlayerInventory`)
  //                               \/
  // (`Shooter`,`Children`)
  //                 +
  // [(`ExampleGun`,`Parent`)]
  let inventory = commands
    .spawn((
      Name::new("Inventory"),
      TransformBundle {
        local: Transform::from_xyz(1.0, -0.8, -1.0),
        ..default()
      },
      Inventory::new(2),
      Shooter::default(),
    ))
    .with_children(|parent| {
      // TODO: Bundle化する
      parent.spawn((
        Name::new("TestGun"),
        // SpatialBundle::default(),
        ExampleGun {
          cool_time: 1.0,
          bullet_speed: 140.0,
          bullet_lifetime: 10.0,
        },
        SceneBundle {
          scene: asset_server.load("models/test_gun2.glb#Scene0"),
          transform: Transform::from_scale(Vec3::splat(4.0)),
          ..default()
        },
        AnimationPlayer::default(),
      ));

      parent.spawn((
        Name::new("TestGun 2"),
        SpatialBundle::default(),
        ExampleGun {
          cool_time: 1.0,
          bullet_speed: 3.0,
          bullet_lifetime: 20.0,
        },
      ));
    })
    .id();

  let player = commands
    .spawn((
      Name::new("Player"),
      Player {
        horizontal_speed: 8.0,
        vertical_speed: 18.0,
        jump_cool_time: 1.2,
        ..default()
      },
      Collider::capsule_y(PLAYER_HALF_HEIGHT, PLAYER_RADIUS),
      PbrBundle {
        mesh: meshes.add(Capsule3d::new(PLAYER_RADIUS, PLAYER_HALF_HEIGHT * 2.0)),
        material: materials.add(Color::Srgba(css::LIGHT_CYAN)),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..default()
      },
      RigidBody::KinematicVelocityBased,
      KinematicCharacterController {
        up: Vec3::Y,
        offset: CharacterLength::Absolute(PLAYER_OFFSET),
        snap_to_ground: Some(CharacterLength::Absolute(0.4)),
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
  commands.entity(camera).push_children(&[body, inventory]);
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
