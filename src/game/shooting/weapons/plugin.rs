use bevy::prelude::*;

use super::{test_gun, TestGun};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<TestGun>()
      .add_systems(Update, test_gun::update);
  }
}
