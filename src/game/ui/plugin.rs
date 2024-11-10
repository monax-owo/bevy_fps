use bevy::prelude::*;

use crate::game::state::GameState;

use super::{
  crosshair::{init_crosshair, spawn_crosshair},
  despawn_ui, spawn_ui, MainMenu,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::PauseMenu), spawn_ui)
      .add_systems(OnExit(GameState::PauseMenu), despawn_ui)
      .add_systems(Startup, spawn_crosshair.after(init_crosshair))
      .register_type::<MainMenu>();
  }
}
