use bevy::prelude::*;
use bevy_rapier3d::{
  plugin::{NoUserData, RapierPhysicsPlugin},
  render::RapierDebugRenderPlugin,
};

use super::{
  player::plugin::PlayerPlugin, shader::plugin::ShaderPlugin, test::plugin::TestPlugin,
  ui::plugin::UiPlugin, world::plugin::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    let app = app.add_plugins((
      RapierPhysicsPlugin::<NoUserData>::default(),
      RapierDebugRenderPlugin::default(),
      PlayerPlugin,
      ShaderPlugin,
      UiPlugin,
      WorldPlugin,
    ));

    if cfg!(debug_assertions) {
      app.add_plugins(TestPlugin);
    }
  }
}
