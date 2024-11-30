use bevy::prelude::*;
pub enum InventoryError {
  Overflow,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Inventory {
  /// itemsのindex
  pub current_item: usize,
  /// itemsの最大の長さ
  max_count: usize,
}

impl Default for Inventory {
  fn default() -> Self {
    Self {
      current_item: Default::default(),
      max_count: 1,
    }
  }
}

impl Inventory {
  pub fn new(max_count: usize) -> Self {
    Self {
      max_count,
      ..Default::default()
    }
  }

  pub fn max_count(&self) -> usize {
    self.max_count
  }

  /// max_countとitemsを指定の長さにし、
  /// 切り詰められた`Option<Item>`を返す
  pub fn set_max_count(&mut self, value: usize) {
    if self.max_count > value {
      // Err?
      todo!();
    }
    self.max_count = value;
  }
}

#[derive(Reflect, Debug)]
pub struct Item(pub Entity);

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Inventory>().register_type::<Item>();
  }
}
