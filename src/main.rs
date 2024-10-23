pub mod game;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
// use bevy_editor_pls::EditorPlugin;
use game::plugin::GamePlugin;
// use bevy_rapier3d::prelude::*;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins,
      FrameTimeDiagnosticsPlugin::default(),
      // EditorPlugin::default(),
      GamePlugin,
    ))
    .run();
}
