use bevy::prelude::*;

use super::{on_enter, on_exit, update_menu, GameState};

pub struct StatePlugin;

impl Plugin for StatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::InGame), on_enter)
      .add_systems(OnExit(GameState::InGame), on_exit)
      .add_systems(Update, update_menu)
      .init_state::<GameState>();
  }
}
