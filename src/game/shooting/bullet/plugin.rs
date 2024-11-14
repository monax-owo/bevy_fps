use bevy::prelude::*;

use super::{
  projectile::{init_projectile, update_projectile, ProjectileBullet, ProjectileBulletAssets},
  raycast::{init_raycast, update_raycast, RaycastBullet, RaycastBulletAssets},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (init_raycast, init_projectile))
      .add_systems(Update, (update_raycast, update_projectile))
      .init_resource::<ProjectileBulletAssets>()
      .init_resource::<RaycastBulletAssets>()
      .register_type::<ProjectileBullet>()
      .register_type::<RaycastBullet>();
  }
}
