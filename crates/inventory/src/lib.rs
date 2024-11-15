use bevy::prelude::*;
pub enum InventoryError {
  Overflow,
}

#[derive(Component, Reflect, Debug)]
pub struct Inventory<T = Item> {
  /// インベントリが保持しているアイテム
  /// Noneは空のスロットを指す
  pub items: Vec<Option<T>>,
  // TODO:要らなくない？
  /// itemsのindex
  pub current_item: usize,
  /// itemsの最大の長さ
  max_count: usize,
}

impl<T> Default for Inventory<T> {
  fn default() -> Self {
    Self {
      items: Default::default(),
      current_item: Default::default(),
      max_count: 1,
    }
  }
}

impl Inventory<Item> {
  pub fn new(max_count: usize) -> Self {
    Self {
      max_count,
      ..Default::default()
    }
  }
}

// TODO
impl<T> Inventory<T> {
  /// itemをインベントリに追加する
  /// # Errors
  /// itemsの長さがmax_count以上の場合に`InventoryError::Overflow`を返す
  pub fn push(&mut self, item: T) -> Result<(), InventoryError> {
    if self.items.len() >= self.max_count {
      return Err(InventoryError::Overflow);
    }
    self.items.push(Some(item));
    Ok(())
  }

  //
  pub fn equip(&mut self, _item: T) -> Result<T, InventoryError> {
    todo!();
    // Ok(item)
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
