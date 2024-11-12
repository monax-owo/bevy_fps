use bevy::prelude::*;

use super::{
  projectile::{init_projectile, update_projectile, ProjectileBullet, ProjectileBulletAssets},
  raycast::{init_raycast, update_raycast, RaycastBullet, RaycastBulletAssets},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (init_projectile, init_raycast))
      .add_systems(Update, (update_projectile, update_raycast))
      .init_resource::<ProjectileBulletAssets>()
      .init_resource::<RaycastBulletAssets>()
      .register_type::<ProjectileBullet>()
      .register_type::<RaycastBullet>();
  }
}
