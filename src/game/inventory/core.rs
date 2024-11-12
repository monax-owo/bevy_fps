use bevy::prelude::*;

pub enum InventoryError {
  Overflow,
}

#[derive(Component, Reflect, Debug, Default)]
pub struct PlayerInventory {
  pub inventory: Inventory,
  /// current_itemのモデルを表示させるエンティティ
  /// `None`の場合はこのコンポーネントを持っているエンティティを指す
  pub model_applier: Option<Entity>,
}

impl PlayerInventory {
  pub fn new(inventory: Inventory, model_applier: Entity) -> Self {
    Self {
      inventory,
      model_applier: Some(model_applier),
    }
  }
}

pub(super) fn update_model() {
  todo!();
  // TODO: current_itemのモデルを表示させる
  //   PbrBundle {
  //   mesh: meshes.add(Cuboid::new(0.2, 0.4, 0.8)),
  //   material: materials.add(Color::Srgba(css::DARK_GRAY)),
  //   ..default()
  // },
}

#[derive(Reflect, Debug)]
pub struct Inventory {
  /// インベントリが保持しているアイテム
  /// Noneは空のスロットを指す
  pub items: Vec<Option<Item>>,
  /// itemsのindex
  pub current_item: usize,
  /// itemsの最大の長さ
  max_count: usize,
}

impl Default for Inventory {
  fn default() -> Self {
    Self {
      items: Default::default(),
      current_item: Default::default(),
      max_count: 1,
    }
  }
}

// TODO
impl Inventory {
  pub fn new(items: Vec<Item>, max_count: usize) -> Self {
    Self {
      items: items.into_iter().map(|v| Some(v)).collect(),
      current_item: Default::default(),
      max_count,
    }
  }

  /// itemをインベントリに追加する
  /// # Errors
  /// itemsの長さがmax_count以上の場合に`InventoryError::Overflow`を返す
  pub fn push(&mut self, item: Item) -> Result<(), InventoryError> {
    if self.items.len() >= self.max_count {
      return Err(InventoryError::Overflow);
    }
    self.items.push(Some(item));
    Ok(())
  }

  //
  pub fn equip(&mut self, item: Item) -> Result<Item, InventoryError> {
    todo!();
    Ok(item)
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
pub struct Item(Entity);
