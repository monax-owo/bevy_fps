use bevy::{color::palettes::css, prelude::*};

// TODO:Bundle化して公開範囲を狭める
#[derive(Component, Reflect, Debug)]
pub struct RaycastBullet {
  pub axis: Dir3,
  /// m/sec
  pub speed: f32,
  /// 銃弾が消滅するまでの時間
  pub lifetime: Timer,
}

// TODO:公開範囲を狭める
#[derive(Resource, Debug, Default)]
pub struct RaycastBulletAssets {
  pub bullet_mesh: Handle<Mesh>,
  pub bullet_material: Handle<StandardMaterial>,
}

#[derive(Resource, Debug)]
pub struct RaycastBulletGroup(pub Entity);

pub(super) fn init_raycast(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Cuboid::from_length(0.2));
  let material = materials.add(Color::Srgba(css::BROWN));
  commands.insert_resource(RaycastBulletAssets {
    bullet_mesh: mesh,
    bullet_material: material,
  });

  let group = commands
    .spawn((Name::new("RaycastBulletGroup"), SpatialBundle::default()))
    .id();

  commands.insert_resource(RaycastBulletGroup(group));
}

// TODO:ヒットスキャン
pub(super) fn update_raycast(
  mut _commands: Commands,
  _time: Res<Time>,
  mut bullet_query: Query<(Entity, &mut RaycastBullet, &mut Transform)>,
) {
  for (_entity, mut _bullet, mut _transform) in bullet_query.iter_mut() {
    // TODO:raycast
    todo!();
  }

  // TODO:判定を通り抜けないように進行方向の軸に対してrayを伸ばす
  // もしくは毎フレーム進む距離だけRaycastする
}
