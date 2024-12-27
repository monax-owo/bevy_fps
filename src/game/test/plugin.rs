use bevy::prelude::*;

use super::{
  hot_reload, init_tester, update_tester,
  world::{generate_collider, init_world},
  TestTag,
};

pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut App) {
    let systems = (update_tester, generate_collider, hot_reload);
    app
      .add_systems(Startup, (init_tester, init_world))
      .add_systems(Update, systems)
      .register_type::<TestTag>();
  }
}
