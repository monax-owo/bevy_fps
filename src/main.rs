pub mod game;

use bevy::prelude::*;
use game::plugin::GamePlugin;
// use bevy_rapier3d::prelude::*;

fn main() {
  let mut app = App::new();
  app.add_plugins((DefaultPlugins, GamePlugin));

  if true {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugins(EditorPlugin::default());
  }

  app.run();
}
