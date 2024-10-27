use bevy::prelude::*;

use super::world::init_world;

pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, (init_world,));
  }
}
