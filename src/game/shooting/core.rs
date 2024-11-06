use bevy::prelude::*;

use crate::game::player::MovementInput;

#[derive(Component, Reflect, Debug)]
pub(super) struct Shooter {
  firing_rate: f32,
}

pub(super) fn update_shooter(key: Res<MovementInput>, shooter_query: Query<&Shooter>) {
  for shooter in shooter_query.iter() {}
}
