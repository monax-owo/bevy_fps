use bevy::prelude::*;

use crate::game::shooting::core::Weapon;

pub struct TestGun {
  pub cool_time: f32,
}

impl Weapon for TestGun {
  fn left_click(&self) {}

  fn right_click(&self) {}

  fn firing_rate(&self) -> f32 {
    1.0
  }

  fn update(&self, world: &mut World) {
    let _ = world;
  }
}
