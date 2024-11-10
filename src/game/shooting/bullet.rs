use bevy::{color::palettes::css, prelude::*};

#[derive(Component, Reflect, Debug)]
pub(super) struct Bullet {
  pub axis: Dir3,
  pub speed: f32,
  pub lifetime: f32,
}

#[derive(Resource, Debug, Default)]
pub(super) struct BulletAssets {
  pub bullet_mesh: Handle<Mesh>,
  pub bullet_material: Handle<StandardMaterial>,
}

pub(super) fn init_bullet(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Sphere::new(0.4));
  let material = materials.add(Color::Srgba(css::BROWN));
  commands.insert_resource(BulletAssets {
    bullet_mesh: mesh,
    bullet_material: material,
  });
}

pub(super) fn update_bullet(
  mut commands: Commands,
  time: Res<Time>,
  mut bullet_query: Query<(Entity, &mut Bullet, &mut Transform)>,
) {
  for (entity, mut bullet, mut transform) in bullet_query.iter_mut() {
    transform.translation += bullet.axis * bullet.speed * time.delta_seconds();
    bullet.lifetime -= time.delta_seconds();

    if bullet.lifetime <= 0.0 {
      commands.entity(entity).despawn();
    }
  }
}
