use bevy::prelude::*;

use super::{update_menu, GameState};

pub struct StatePlugin;

impl Plugin for StatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (update_menu,))
      .init_state::<GameState>();
  }
}
