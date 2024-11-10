use bevy::prelude::*;

use crate::game::player::input::PlayerInput;

#[derive(Component, Reflect, Debug, Default)]
pub struct Shooter {}

#[derive(Event, Debug)]
pub struct FireEvent;

pub(super) fn init_shooter(mut _commands: Commands) {}

pub(super) fn update_shooter(
  mut fire_event_writer: EventWriter<FireEvent>,
  input: Res<PlayerInput>,
  shooter_query: Query<&Shooter>,
) {
  for _shooter in shooter_query.iter() {
    if input.fire {
      fire_event_writer.send(FireEvent);
    }
  }
}

// TODO:ヒットスキャン
pub(super) fn _update_hit_scan() {}
