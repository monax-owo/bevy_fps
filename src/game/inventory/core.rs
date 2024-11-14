use std::any::Any;

use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Default)]
pub struct PlayerInventory {
  /// current_itemのモデルを表示させるエンティティ
  /// `None`の場合はこのコンポーネントを持っているエンティティを指す
  pub item_user: Option<Entity>,
  #[reflect(ignore)]
  pub current_item_type: Box<dyn Any + Send + Sync + 'static>,
}

impl Default for PlayerInventory {
  fn default() -> Self {
    Self {
      item_user: Default::default(),
      current_item_type: Box::new(()),
    }
  }
}

impl PlayerInventory {
  pub fn new(model_applier: Entity) -> Self {
    Self {
      item_user: Some(model_applier),
      current_item_type: Box::new(()),
    }
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
