use bevy::prelude::*;

use super::{
  core::TestTag,
  world::{generate_collider, init_world},
};

pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<TestTag>()
      .add_systems(Startup, (init_world,))
      .add_systems(Update, (generate_collider,));
  }
}
