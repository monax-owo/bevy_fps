use bevy::prelude::*;

use super::camera_controller::CameraController;

#[derive(Debug, Component)]
pub struct Player;

pub fn init_player(mut commands: Commands) {
  commands
    .spawn((
      Player,
      SpatialBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
      },
    ))
    .with_children(|parent| {
      parent.spawn((
        Camera3dBundle {
          camera: Camera {
            order: 1,
            ..default()
          },
          projection: Projection::Perspective(PerspectiveProjection {
            fov: 70.0,
            ..default()
          }),
          ..default()
        },
        CameraController { sensitivity: 0.001 },
      ));
    });
}
