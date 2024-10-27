use bevy::prelude::*;

use super::{
  player::plugin::PlayerPlugin, shader::plugin::ShaderPlugin, test::plugin::TestPlugin,
  ui::plugin::UiPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    let app = app.add_plugins((UiPlugin, PlayerPlugin, ShaderPlugin));

    if cfg!(debug_assertions) {
      app.add_plugins(TestPlugin);
    }
  }
}
