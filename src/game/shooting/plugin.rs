use bevy::prelude::*;

use super::{
  bullet::{init_bullet, Bullet, BulletAssets},
  init_shooter, update_shooter,
  weapons::WeaponPlugin,
  Shooter,
};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(WeaponPlugin)
      .init_resource::<BulletAssets>()
      .register_type::<Bullet>()
      .register_type::<Shooter>()
      .add_systems(Startup, (init_shooter, init_bullet))
      .add_systems(Update, update_shooter);
  }
}
