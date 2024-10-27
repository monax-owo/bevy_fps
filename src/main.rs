pub mod game;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use game::plugin::GamePlugin;
// use bevy_rapier3d::prelude::*;

fn main() {
  let mut app = App::new();
  app.add_plugins((
    DefaultPlugins,
    FrameTimeDiagnosticsPlugin::default(),
    GamePlugin,
  ));

  if true {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugins(EditorPlugin::default());
  }

  app.run();
}
