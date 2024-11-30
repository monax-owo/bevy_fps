use bevy::prelude::*;
use inventory::Inventory;

use crate::game::player::input::PlayerInput;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct CurrentWeapon;

// TODO: フローチャートにする
pub(super) fn update_current_item(
  mut commands: Commands,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  key: Res<PlayerInput>,
  mut inventory_query: Query<(&mut Inventory, &Children)>,
  current_item_query: Query<Entity, With<CurrentWeapon>>,
  mut visibility_query: Query<&mut Visibility>,
) {
  for (mut inventory, children) in inventory_query.iter_mut() {
    if keyboard_input.just_pressed(key.item_1) {
      inventory.current_item = 1;
    }
    if keyboard_input.just_pressed(key.item_2) {
      inventory.current_item = 2;
    }
    if keyboard_input.just_pressed(key.item_3) {
      inventory.current_item = if inventory.current_item != 0 { 0 } else { 1 };
    }

    if inventory.current_item > inventory.max_count() {
      inventory.current_item = inventory.max_count();
    }

    // childrenからcurrent_item_queryに当てはまるエンティティを探す
    let find_current_item: Vec<&Entity> = children
      .into_iter()
      .filter(|v| current_item_query.get(**v).is_ok())
      .collect();

    //
    let _current_weapon = match find_current_item.len() {
      1 => *find_current_item[0],
      0 => {
        // childrenの最初のエンティティに付与
        inventory.current_item = 0;
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

    // TODO: 上のコードと重複している気がする
    // TODO: Localを使ってinventory.current_itemが変更されたときのみ実行する
    for (i, child) in children.iter().enumerate() {
      if let Ok(mut visibility) = visibility_query.get_mut(*child) {
        if i == inventory.current_item {
          commands.entity(*child).insert(CurrentWeapon);
          *visibility = Visibility::Inherited;
        } else {
          commands.entity(*child).remove::<CurrentWeapon>();
          *visibility = Visibility::Hidden;
        }
      }
    }
  }
}
