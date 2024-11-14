use bevy::prelude::*;

use super::{update_children, update_model, PlayerInventory};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (update_children, update_model))
      .register_type::<PlayerInventory>();
  }
}
