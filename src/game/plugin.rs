use bevy::prelude::*;
use bevy_rapier3d::{
  plugin::{NoUserData, RapierPhysicsPlugin},
  render::RapierDebugRenderPlugin,
};

use super::{
  inventory::plugin::InventoryPlugin, player::PlayerPlugin, shader::ShaderPlugin,
  shooting::ShootingPlugin, state::StatePlugin, test::TestPlugin, ui::UiPlugin, world::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    let app = app.add_plugins((
      RapierPhysicsPlugin::<NoUserData>::default(),
      // inventory crateのPlugin
      inventory::InventoryPlugin,
      InventoryPlugin,
      PlayerPlugin,
      ShaderPlugin,
      ShootingPlugin,
      StatePlugin,
      UiPlugin,
      WorldPlugin,
    ));

    if cfg!(debug_assertions) {
      app.add_plugins((TestPlugin, RapierDebugRenderPlugin::default()));
    }
  }
}
