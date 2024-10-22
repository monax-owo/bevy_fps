pub mod plugin;

use bevy::prelude::*;

#[derive(Debug, Component)]
struct Player;

pub fn init_player(mut commands: Commands) {
  commands
    .spawn((
      Player,
      SpatialBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
      },
    ))
    .with_children(|parent| {
      parent.spawn(Camera3dBundle {
        camera: Camera {
          order: 1,
          ..default()
        },
        projection: Projection::Perspective(PerspectiveProjection {
          fov: 90.0,
          ..default()
        }),
        ..default()
      });
    });
}
