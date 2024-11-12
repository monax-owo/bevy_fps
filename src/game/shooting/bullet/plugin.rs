use bevy::prelude::*;

use super::{
  projectile::{
    init_projectile_bullet, update_projectile_bullet, ProjectileBullet, ProjectileBulletAssets,
  },
  raycast::{init_raycast_bullet, update_raycast_bullet, RaycastBullet, RaycastBulletAssets},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (init_projectile_bullet, init_raycast_bullet))
      .add_systems(Update, (update_projectile_bullet, update_raycast_bullet))
      .init_resource::<ProjectileBulletAssets>()
      .init_resource::<RaycastBulletAssets>()
      .register_type::<ProjectileBullet>()
      .register_type::<RaycastBullet>();
  }
}
