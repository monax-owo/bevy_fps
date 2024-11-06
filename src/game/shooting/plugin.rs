use bevy::prelude::*;

use super::core::{init_shooter, update_shooter, ShooterAssets};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<ShooterAssets>()
      .add_systems(Startup, init_shooter)
      .add_systems(Update, update_shooter);
  }
}
