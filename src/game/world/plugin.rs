use bevy::prelude::*;

use super::world::init_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_world);
  }
}
