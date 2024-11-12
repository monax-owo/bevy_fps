use bevy::prelude::*;

use super::{
  projectile::{
    init_projectile_bullet, update_projectile_bullet, ProjectileBullet, ProjectileBulletAssets,
  },
  raycast::{RaycastBullet, RaycastBulletAssets},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, init_projectile_bullet)
      .add_systems(Update, update_projectile_bullet)
      .init_resource::<ProjectileBulletAssets>()
      .init_resource::<RaycastBulletAssets>()
      .register_type::<ProjectileBullet>()
      .register_type::<RaycastBullet>();
  }
}
