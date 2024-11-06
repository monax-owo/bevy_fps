use bevy::prelude::*;

use super::core::update_shooter;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, update_shooter);
  }
}
