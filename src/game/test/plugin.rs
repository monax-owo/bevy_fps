use bevy::prelude::*;

use super::{
  init_tester, update_2, update_tester,
  world::{generate_collider, init_world},
  TestTag,
};

pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (init_world, init_tester))
      .add_systems(Update, (generate_collider, update_tester, update_2))
      .register_type::<TestTag>();
  }
}
