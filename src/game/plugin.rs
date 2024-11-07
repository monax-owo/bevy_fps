use bevy::prelude::*;
use bevy_rapier3d::{
  plugin::{NoUserData, RapierPhysicsPlugin},
  render::RapierDebugRenderPlugin,
};

use super::{
  player::plugin::PlayerPlugin, shader::plugin::ShaderPlugin, shooting::plugin::ShootingPlugin,
  state::plugin::StatePlugin, test::plugin::TestPlugin, ui::plugin::UiPlugin,
  world::plugin::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    let app = app.add_plugins((
      RapierPhysicsPlugin::<NoUserData>::default(),
      RapierDebugRenderPlugin::default(),
      PlayerPlugin,
      ShaderPlugin,
      ShootingPlugin,
      StatePlugin,
      UiPlugin,
      WorldPlugin,
    ));

    if cfg!(debug_assertions) {
      app.add_plugins(TestPlugin);
    }
  }
}
