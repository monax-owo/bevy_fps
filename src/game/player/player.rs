use bevy::{core_pipeline::tonemapping::DebandDither, prelude::*};

use super::camera_controller::CameraController;

#[derive(Component)]
pub(super) struct Player {
  pub speed: f32,
}

pub(super) fn init_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn((
      Player { speed: 8.0 },
      PbrBundle {
        mesh: meshes.add(Capsule3d::new(0.6, 1.4)),
        material: materials.add(Color::srgb_u8(0, 255, 255)),
        transform: Transform::from_xyz(0.0, 1.3, 0.0),
        ..default()
      },
    ))
    .with_children(|parent| {
      parent.spawn((
        Camera3dBundle {
          deband_dither: DebandDither::Disabled,
          camera: Camera {
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
