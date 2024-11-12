use bevy::prelude::*;

pub enum InventoryError {
  Overflow,
}

#[derive(Component, Reflect, Debug)]
pub struct Inventory {
  pub items: Vec<Option<Item>>,
  pub current: usize,
  max_count: usize,
}

// TODO
impl Inventory {
  fn new(items: Vec<Item>, max_count: usize) -> Self {
    Self {
      items: items.into_iter().map(|v| Some(v)).collect(),
      current: Default::default(),
      max_count,
    }
  }

  /// itemをインベントリに追加する
  /// # Errors
  /// itemsの長さがmax_count以上の場合に`InventoryError::Overflow`を返す
  fn add(&mut self, item: Item) -> Result<(), InventoryError> {
    if self.items.len() >= self.max_count {
      return Err(InventoryError::Overflow);
    }
    self.items.push(Some(item));
    Ok(())
  }

  //
  fn equip(&mut self, item: Item) -> Result<Item, InventoryError> {
    todo!();
    Ok(item)
  }

  fn max_count(&self) -> usize {
    self.max_count
  }

  /// max_countとitemsを指定の長さにし、
  /// 切り詰められた`Option<Item>`を返す
  fn set_max_count(&mut self, value: usize) {
    if self.max_count > value {
      // Err?
      todo!();
    }
    self.max_count = value;
  }
}

impl Default for Inventory {
  fn default() -> Self {
    Self {
      items: Default::default(),
      current: Default::default(),
      max_count: 1,
    }
  }
}

#[derive(Reflect, Debug)]
pub struct Item(Entity);
