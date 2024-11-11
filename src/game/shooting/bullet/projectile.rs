use bevy::{color::palettes::css, prelude::*};

// TODO:Bundle化して公開範囲を狭める
#[derive(Component, Reflect, Debug)]
pub struct ProjectileBullet {
  pub axis: Dir3,
  /// m/sec
  pub speed: f32,
  /// sec
  pub lifetime: f32,
}

// TODO:公開範囲を狭める
#[derive(Resource, Debug, Default)]
pub struct ProjectileBulletAssets {
  pub bullet_mesh: Handle<Mesh>,
  pub bullet_material: Handle<StandardMaterial>,
}

pub(super) fn init_projectile_bullet(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Sphere::new(0.4));
  let material = materials.add(Color::Srgba(css::BROWN));
  commands.insert_resource(ProjectileBulletAssets {
    bullet_mesh: mesh,
    bullet_material: material,
  });
}

// TODO:ヒットスキャン
pub(super) fn update_projectile_bullet(
  mut commands: Commands,
  time: Res<Time>,
  mut bullet_query: Query<(Entity, &mut ProjectileBullet, &mut Transform)>,
) {
  for (entity, mut bullet, mut transform) in bullet_query.iter_mut() {
    transform.translation += bullet.axis * bullet.speed * time.delta_seconds();
    bullet.lifetime -= time.delta_seconds();

    if bullet.lifetime <= 0.0 {
      commands.entity(entity).despawn();
    }
  }
}
