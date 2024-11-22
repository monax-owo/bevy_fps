pub mod game;

use std::env;

use bevy::{prelude::*, window::PresentMode};
use game::GamePlugin;

fn main() {
  let mut app = App::new();
  app.add_plugins((
    DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "bevy_fps".into(),
        present_mode: PresentMode::AutoNoVsync,
        ..default()
      }),
      ..default()
    }),
    GamePlugin,
  ));

  if env::args().any(|v| &v == "--gui") {
    use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    use bevy_editor_pls::EditorPlugin;

    app.add_plugins((
      EditorPlugin::default(),
      FrameTimeDiagnosticsPlugin::default(),
    ));
  }

  app.run();
}
