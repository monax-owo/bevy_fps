use bevy::prelude::*;

use super::{update_item, PlayerInventory};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_item)
      .register_type::<PlayerInventory>();
  }
}
