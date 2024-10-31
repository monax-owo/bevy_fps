use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub(super) fn init_world(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  const SPHERE_SIZE: f32 = 0.04;

  // AmbientLight
  commands.insert_resource(AmbientLight {
    color: Color::srgb_u8(210, 220, 240),
    brightness: 280.0,
  });

  // Ground
  commands
    .spawn((
      Name::new("World"),
      Collider::cuboid(100.0, 0.01, 100.0),
      PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0))),
        material: materials.add(Color::srgb_u8(255, 0, 127)),
        ..default()
      },
    ))
    .with_children(|parent| {
      // Box
      parent.spawn((
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
        PbrBundle {
          mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
          material: materials.add(Color::srgb_u8(255, 0, 0)),
          transform: Transform::from_xyz(0.0, 4.0, -4.0),
          ..default()
        },
      ));

      // Gizmo
      parent.spawn(PbrBundle {
        mesh: meshes.add(Rhombus {
          half_diagonals: Vec2::splat(0.1),
        }),
        material: materials.add(Color::srgb_u8(255, 0, 0)),
        ..default()
      });

      // Sphere top
      parent.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(SPHERE_SIZE)),
        material: materials.add(Color::srgb_u8(255, 0, 0)),
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..default()
      });

      // Sphere forward
      parent.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(SPHERE_SIZE)),
        material: materials.add(Color::srgb_u8(255, 0, 0)),
        transform: Transform::from_xyz(0.0, 4.0, -1.0),
        ..default()
      });

      // PointLight
      parent.spawn(PointLightBundle {
        point_light: PointLight {
          color: Color::srgb(1.0, 1.0, 0.8),
          intensity: 600000.0,
          range: 60.0,
          shadows_enabled: true,
          ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
      });
    });
}
