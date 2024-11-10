use bevy::prelude::*;

use crate::game::shooting::update_shooter;

use super::test_gun;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<test_gun::TestGun>()
      .add_systems(Update, (test_gun::update).after(update_shooter));
  }
}
