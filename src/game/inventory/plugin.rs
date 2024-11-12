use bevy::prelude::*;

use super::update_model;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, update_model);
  }
}
