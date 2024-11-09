use bevy::prelude::*;

use super::init_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ClearColor(Color::srgb(0.52, 0.76, 0.88)))
      .add_systems(Startup, init_world);
  }
}
