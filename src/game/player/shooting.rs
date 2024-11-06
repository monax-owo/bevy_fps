use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub(super) struct Shooter {
  firing_rate: f32,
}
