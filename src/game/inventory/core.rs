use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use inventory::{Inventory, Item};

type ItemInventory = Inventory<Item>;

#[derive(Component, Reflect, Debug, Default)]
pub struct PlayerInventory {
  pub inventory: ItemInventory,
  /// current_itemのモデルを表示させるエンティティ
  /// `None`の場合はこのコンポーネントを持っているエンティティを指す
  pub model_applier: Option<Entity>,
}

impl PlayerInventory {
  pub fn new(inventory: ItemInventory, model_applier: Entity) -> Self {
    Self {
      inventory,
      model_applier: Some(model_applier),
    }
  }
}

impl Deref for PlayerInventory {
  type Target = ItemInventory;

  fn deref(&self) -> &Self::Target {
    &self.inventory
  }
}

impl DerefMut for PlayerInventory {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inventory
  }
}

pub(super) fn update_model() {
  // todo!();
  // TODO: current_itemのモデルを表示させる
  //   PbrBundle {
  //   mesh: meshes.add(Cuboid::new(0.2, 0.4, 0.8)),
  //   material: materials.add(Color::Srgba(css::DARK_GRAY)),
  //   ..default()
  // },
}
