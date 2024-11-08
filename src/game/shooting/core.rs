use bevy::prelude::*;

use crate::game::player::input::PlayerInput;

#[derive(Component, Reflect, Debug)]
pub struct Shooter {
  pub weapon: Entity,
}

pub(super) fn init_shooter(mut _commands: Commands) {}

pub(super) fn update_shooter(
  mut _commands: Commands,
  // assets: Res<BulletAssets>,
  input: Res<PlayerInput>,
  shooter_query: Query<(Entity, &Shooter)>,
) {
  for (_entity, _shooter) in shooter_query.iter() {
    if input.fire {
      // TODO:eventを発火
    }
  }
}

// TODO:ヒットスキャン
pub(super) fn _update_hit_scan() {}
