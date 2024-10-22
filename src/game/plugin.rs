use bevy::{
  app::{Plugin, Startup},
  prelude::Component,
};

use super::{player::plugin::PlayerPlugin, world::init_world};

#[derive(Debug, Component)]
pub struct GamePlugin;
impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .add_plugins(PlayerPlugin)
      .add_systems(Startup, init_world);
  }
}
