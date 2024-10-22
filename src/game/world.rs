use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub fn init_world(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((
    Collider::cuboid(10.0, 0.0, 10.0),
    PbrBundle {
      mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0))),
      material: materials.add(Color::srgb_u8(255, 0, 127)),
      ..default()
    },
  ));
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      color: Color::srgb_u8(255, 255, 255),
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(0.0, 10.0, 0.0),
    ..default()
  });
}
