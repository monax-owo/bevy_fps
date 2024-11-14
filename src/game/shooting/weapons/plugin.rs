use bevy::prelude::*;

use crate::game::shooting::update_shooter;

use super::test_gun;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (test_gun::update).after(update_shooter))
      .register_type::<test_gun::ExampleGun>();
  }
}
