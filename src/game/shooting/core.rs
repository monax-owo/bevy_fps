use bevy::prelude::*;

use crate::game::player::input::PlayerInput;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Shooter {}

#[derive(Event, Debug)]
pub struct FireEvent;

pub(super) fn init_shooter(mut _commands: Commands) {}

pub(super) fn update_shooter(
  mouse_input: Res<ButtonInput<MouseButton>>,
  mut fire_event_writer: EventWriter<FireEvent>,
  input: Res<PlayerInput>,
  shooter_query: Query<&Shooter>,
) {
  for _ in shooter_query.iter() {
    if mouse_input.pressed(input.fire) {
      // TODO:EventじゃなくてPlayerInputで良さそう？
      // でも状況によって銃を撃てないようにできたほうがいい…？
      fire_event_writer.send(FireEvent);
    }
  }
}
