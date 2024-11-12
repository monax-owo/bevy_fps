use bevy::prelude::*;

use super::{
  init_tester, update_tester,
  world::{generate_collider, init_world},
  TestTag,
};

pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (init_world, init_tester))
      .add_systems(Update, (generate_collider, update_tester))
      .register_type::<TestTag>();
  }
}
