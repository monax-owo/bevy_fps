use bevy::{
  app::{Plugin, Startup},
  prelude::Component,
};

use super::{
  player::plugin::PlayerPlugin, shader::plugin::ShaderPlugin, test::plugin::TestPlugin,
  ui::plugin::UiPlugin, world::init_world,
};

#[derive(Component)]
pub struct GamePlugin;
impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    let app = app
      .add_plugins((UiPlugin, PlayerPlugin, ShaderPlugin))
      .add_systems(Startup, init_world);

    #[cfg(debug_assertions)]
    {
      app.add_plugins(TestPlugin);
    }
  }
}
