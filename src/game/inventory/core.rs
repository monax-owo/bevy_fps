use std::any::Any;

use bevy::prelude::*;
use inventory::Inventory;

#[derive(Component, Reflect)]
#[reflect(Default)]
pub struct PlayerInventory {
  /// current_itemのモデルを表示させるエンティティ
  /// `None`の場合はこのコンポーネントを持っているエンティティを指す
  pub item_user: Option<Entity>,
  #[reflect(ignore)]
  pub current_item_type: Box<dyn Any + Send + Sync + 'static>,
}

#[derive(Component, Reflect, Debug)]
pub struct CurrentWeapon;

impl Default for PlayerInventory {
  fn default() -> Self {
    Self {
      item_user: Default::default(),
      current_item_type: Box::new(()),
    }
  }
}

impl PlayerInventory {
  pub fn new(item_user: Entity) -> Self {
    Self {
      item_user: Some(item_user),
      ..Default::default()
    }
  }
}

pub(super) fn update_item(
  mut commands: Commands,
  inventory_query: Query<&Children, (With<Inventory>, Changed<Children>)>,
  current_item_query: Query<Entity, With<CurrentWeapon>>,
) {
  for children in inventory_query.into_iter() {
    // childrenからWith<CurrentItem>に当てはまるエンティティを探す
    let find: Vec<&Entity> = children
      .into_iter()
      .filter(|v| current_item_query.get(**v).is_ok())
      .collect();

    match find.len() {
      1 => (),
      0 => {
        dbg!("0");
        // childrenの最初のエンティティに付与
        commands.entity(children[0]).insert(CurrentWeapon);
      }
      _ => {
        dbg!("_");
        // findの最初以外のエンティティから削除
        for (i, v) in find.into_iter().enumerate() {
          if i != 0 {
            commands.entity(*v).remove::<CurrentWeapon>();
          }
        }
      }
    }
  }
}
