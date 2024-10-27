use bevy::prelude::*;

use crate::game::shader::shader::PostProcessMaterial;

pub(super) fn init_world(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<PostProcessMaterial>>,
) {
  commands.spawn(MaterialMeshBundle {
    mesh: meshes.add(Sphere::new(0.2)),
    material: materials.add(PostProcessMaterial {
      color: LinearRgba::WHITE,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 4.0, 0.0),
    ..default()
  });
}
