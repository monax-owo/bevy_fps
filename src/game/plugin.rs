use bevy::{
  app::{Plugin, Startup},
  prelude::Component,
};

use super::{player::plugin::PlayerPlugin, ui::plugin::UiPlugin, world::init_world};

#[derive(Component)]
pub struct GamePlugin;
impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .add_plugins((UiPlugin, PlayerPlugin))
      .add_systems(Startup, init_world);
  }
}
