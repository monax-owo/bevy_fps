use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub fn init_world(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // Ground
  commands.spawn((
    Collider::cuboid(10.0, 0.0, 10.0),
    PbrBundle {
      mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0))),
      material: materials.add(Color::srgb_u8(255, 0, 127)),
      ..default()
    },
  ));
  // Gizmo
  commands.spawn(PbrBundle {
    mesh: meshes.add(Sphere::new(0.1)),
    material: materials.add(Color::srgb_u8(255, 0, 0)),
    ..default()
  });
  // Gizmo top
  commands.spawn(PbrBundle {
    mesh: meshes.add(Sphere::new(0.1)),
    material: materials.add(Color::srgb_u8(255, 0, 0)),
    transform: Transform::from_xyz(0.0, 4.0, 0.0),
    ..default()
  });
  // Gizmo 2
  commands.spawn(PbrBundle {
    mesh: meshes.add(Sphere::new(0.02)),
    material: materials.add(Color::srgb_u8(255, 0, 0)),
    transform: Transform::from_xyz(0.0, 2.0, -1.0),
    ..default()
  });
  // Light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      color: Color::srgb_u8(255, 255, 255),
      shadows_enabled: true,
      intensity: 3000000.0,
      ..default()
    },
    transform: Transform::from_xyz(0.0, 10.0, 2.0),
    ..default()
  });
}
