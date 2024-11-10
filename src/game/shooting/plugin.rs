use bevy::prelude::*;

use super::{
  bullet::{init_bullet, update_bullet, Bullet, BulletAssets},
  init_shooter, update_shooter,
  weapons::WeaponPlugin,
  FireEvent, Shooter,
};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<FireEvent>()
      .add_plugins(WeaponPlugin)
      .add_systems(Startup, (init_shooter, init_bullet))
      .add_systems(Update, (update_shooter, update_bullet))
      .init_resource::<BulletAssets>()
      .register_type::<Bullet>()
      .register_type::<Shooter>();
  }
}
