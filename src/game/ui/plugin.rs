use bevy::prelude::*;

use super::crosshair::init_crosshair;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_crosshair);
  }
}
