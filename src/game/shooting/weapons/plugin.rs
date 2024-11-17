use bevy::prelude::*;

use crate::game::shooting::update_shooter;

use super::{example_gun, ExampleGun};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (example_gun::update).after(update_shooter))
      .register_type::<ExampleGun>();
  }
}
