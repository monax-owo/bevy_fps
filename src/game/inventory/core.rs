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
  pub fn new(model_applier: Entity) -> Self {
    Self {
      inventory: Default::default(),
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
  //   transform: Transform::from_xyz(1.0, -0.4, -1.0),
  //   ..default()
  // },
}

#[derive(Reflect, Debug)]
pub struct Inventory {
  pub items: Vec<Option<Item>>,
  pub current_item: usize,
  max_count: usize,
}

// TODO
impl Inventory {
  fn new(items: Vec<Item>, max_count: usize) -> Self {
    Self {
      items: items.into_iter().map(|v| Some(v)).collect(),
      current_item: Default::default(),
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
      current_item: Default::default(),
      max_count: 1,
    }
  }
}

#[derive(Reflect, Debug)]
pub struct Item(Entity);
