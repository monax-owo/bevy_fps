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
  for _ in shooter_query.iter() {
    if input.fire {
      // TODO:EventじゃなくてPlayerInputで良さそう？
      // でも状況によって銃を撃てないようにできたほうがいい…？
      fire_event_writer.send(FireEvent);
    }
  }
}

// TODO:
// pub(super) fn update_gun_model(last: Local<Item>, mut gun_query: Query<(Entity, &Shooter)>) {
//   for (entity, shooter) in &gun_query {}
// }
