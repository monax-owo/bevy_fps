use bevy::prelude::*;

use crate::game::state::GameState;

use super::{
  core::{despawn_ui, spawn_ui},
  crosshair::{init_crosshair, spawn_crosshair},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_crosshair.after(init_crosshair))
      .add_systems(OnEnter(GameState::MainMenu), spawn_ui)
      .add_systems(OnExit(GameState::MainMenu), despawn_ui);
  }
}
