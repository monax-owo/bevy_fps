use std::any::Any;

use bevy::prelude::*;
use inventory::Inventory;

use crate::game::player::input::PlayerInput;

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

pub(super) fn update_current_item(
  mut commands: Commands,
  key: Res<PlayerInput>,
  mut inventory_query: Query<(&mut Inventory, &Children)>,
  current_item_query: Query<Entity, With<CurrentWeapon>>,
) {
  for (mut inventory, children) in inventory_query.iter_mut() {
    inventory.current_item = inventory.current_item.clamp(0, inventory.max_count());

    if key.blink {
      dbg!(inventory.current_item);
      inventory.current_item = if inventory.current_item != 0 { 0 } else { 1 };
    }

    // childrenからcurrent_item_queryに当てはまるエンティティを探す
    let find: Vec<&Entity> = children
      .into_iter()
      .filter(|v| current_item_query.get(**v).is_ok())
      .collect();

    // TODO: CurrentWeaponを変えれるようにする
    let current_weapon = match find.len() {
      1 => *find[0],
      0 => {
        dbg!("0");
        // childrenの最初のエンティティに付与
        commands.entity(children[0]).insert(CurrentWeapon).id()
      }
      _ => {
        dbg!("_");
        // findの最初以外のエンティティから削除
        let mut iter = find.into_iter();
        let next = iter.next();
        iter.enumerate().for_each(|(i, e)| {
          commands.entity(*e).remove::<CurrentWeapon>();
        });
        *next.unwrap_or(&children[0])
      }
    };
    //

    //
  }
}
