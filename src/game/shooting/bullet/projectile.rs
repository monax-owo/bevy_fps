use bevy::{color::palettes::css, prelude::*};

// TODO:Bundle化して公開範囲を狭める
#[derive(Component, Reflect, Debug)]
pub struct ProjectileBullet {
  /// m/sec
  pub speed: f32,
  /// sec
  pub lifetime: f32,
}

#[derive(Bundle)]
pub struct ProjectileBulletBundle {
  #[bundle()]
  pbr_bundle: PbrBundle,
  projectile_bullet: ProjectileBullet,
}

// TODO:バンドル化+弾の向き修正(x軸90deg?)
impl ProjectileBulletBundle {
  pub fn new(
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    speed: f32,
    lifetime: f32,
  ) -> Self {
    Self {
      pbr_bundle: PbrBundle {
        mesh,
        material,
        transform,
        ..default()
      },
      projectile_bullet: ProjectileBullet { speed, lifetime },
    }
  }
}

// TODO:公開範囲を狭める
#[derive(Resource, Debug, Default)]
pub struct ProjectileBulletAssets {
  pub bullet_mesh: Handle<Mesh>,
  pub bullet_material: Handle<StandardMaterial>,
}

#[derive(Resource, Debug)]
pub struct ProjectileBulletGroup(pub Entity);

pub(super) fn init_projectile(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Capsule3d::new(0.2, 0.6));
  let material = materials.add(Color::Srgba(css::BROWN));
  commands.insert_resource(ProjectileBulletAssets {
    bullet_mesh: mesh,
    bullet_material: material,
  });

  let group = commands
    .spawn((Name::new("ProjectileBulletGroup"), SpatialBundle::default()))
    .id();

  commands.insert_resource(ProjectileBulletGroup(group));
}

// TODO:ヒットスキャン
pub(super) fn update_projectile(
  mut commands: Commands,
  time: Res<Time>,
  mut bullet_query: Query<(Entity, &mut ProjectileBullet, &mut Transform)>,
) {
  for (entity, mut bullet, mut transform) in bullet_query.iter_mut() {
    transform.translation =
      transform.translation + transform.forward() * bullet.speed * time.delta_seconds();
    bullet.lifetime -= time.delta_seconds();

    if bullet.lifetime <= 0.0 {
      commands.entity(entity).despawn();
    }
  }
}
