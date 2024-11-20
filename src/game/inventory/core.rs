use bevy::prelude::*;
use inventory::Inventory;

use crate::game::player::input::PlayerInput;

#[derive(Component, Reflect, Debug)]
pub struct CurrentWeapon;

// TODO: フローチャートにする
pub(super) fn update_current_item(
  mut commands: Commands,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  key: Res<PlayerInput>,
  mut inventory_query: Query<(&mut Inventory, &Children)>,
  current_item_query: Query<Entity, With<CurrentWeapon>>,
) {
  for (mut inventory, children) in inventory_query.iter_mut() {
    if inventory.current_item > inventory.max_count() {
      inventory.current_item = inventory.max_count();
    }

    if keyboard_input.just_pressed(key.blink) {
      dbg!(inventory.current_item);
      inventory.current_item = if inventory.current_item != 0 { 0 } else { 1 };
    }

    // childrenからcurrent_item_queryに当てはまるエンティティを探す
    let find_current_item: Vec<&Entity> = children
      .into_iter()
      .filter(|v| current_item_query.get(**v).is_ok())
      .collect();

    // TODO: CurrentWeaponを変えれるようにする
    let _current_weapon = match find_current_item.len() {
      1 => *find_current_item[0],
      0 => {
        dbg!("0");
        // childrenの最初のエンティティに付与
        commands.entity(children[0]).insert(CurrentWeapon).id()
      }
      _ => {
        dbg!("_");
        // findの最初以外のエンティティから削除
        let mut iter = find_current_item.into_iter();
        let next = iter.next();
        iter.for_each(|e| {
          commands.entity(*e).remove::<CurrentWeapon>();
        });
        *next.unwrap_or(&children[0])
      }
    };
    //

    //
  }
}
