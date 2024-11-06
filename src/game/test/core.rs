use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub(super) struct TestTag(pub String);
