use bevy::prelude::*;

use crate::game::shooting::bullet::{Bullet, BulletAssets};

#[derive(Component, Reflect, Debug, Default)]
pub struct TestGun {
  pub cool_time: f32,
}

pub(super) fn update(
  mut commands: Commands,
  assets: Res<BulletAssets>,
  gun: Query<(Entity, &TestGun, &Transform)>,
) {
  for (entity, gun, transform) in gun.iter() {
    println!("{}", transform.translation);
    // TODO:弾を発射
    // commands.entity(entity).with_children(|parent| {
    //   parent.spawn((
    //     PbrBundle {
    //       mesh: assets.bullet_mesh.clone(),
    //       material: assets.bullet_material.clone(),
    //       ..default()
    //     },
    //     Bullet { lifetime: 10.0 },
    //   ));
    // });
  }
}
