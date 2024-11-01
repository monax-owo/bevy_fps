use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
pub(super) struct TestTag(pub String);
