use bevy::prelude::*;

use super::{
  bullet::BulletPlugin, init_shooter, update_shooter, weapons::WeaponPlugin, FireEvent, Shooter,
};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<FireEvent>()
      .add_plugins((BulletPlugin, WeaponPlugin))
      .add_systems(Startup, init_shooter)
      .add_systems(Update, update_shooter)
      .register_type::<Shooter>();
  }
}
