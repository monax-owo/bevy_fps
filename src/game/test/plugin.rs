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
      .add_systems(Startup, (init_tester, init_world))
      .add_systems(Update, (update_2, update_tester, generate_collider))
      .register_type::<TestTag>();
  }
}
