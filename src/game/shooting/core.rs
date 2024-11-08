use std::fmt::Debug;

use bevy::prelude::*;

use crate::game::player::input::PlayerInput;

// use super::bullet::{Bullet, BulletAssets};

pub(super) trait Weapon {
  fn left_click(&self) {}
  fn right_click(&self) {}
  fn firing_rate(&self) -> f32 {
    1.0
  }
  fn update(&self, world: &mut World) {}
}

// todo:ReflectとDebugを実装したい
#[derive(Component)]
pub struct Shooter {
  weapon: Box<dyn Weapon + Send + Sync>,
}

impl Default for Shooter {
  fn default() -> Self {
    Self {
      weapon: Box::new(Hand),
    }
  }
}

#[derive(Reflect, Debug)]
pub(super) struct Hand;

impl Weapon for Hand {
  fn left_click(&self) {
    println!("fire!");
  }

  fn right_click(&self) {}

  fn firing_rate(&self) -> f32 {
    1.0
  }
}

pub(super) struct Gun {
  cool_time: f32,
}

impl Weapon for Gun {
  fn left_click(&self) {
    println!("fire");
  }

  fn right_click(&self) {
    todo!()
  }

  fn firing_rate(&self) -> f32 {
    todo!()
  }

  fn update(&self, world: &mut World) {
    world.query::<(Entity, &Transform)>();
  }
}

pub(super) fn init_shooter(mut _commands: Commands) {}

pub(super) fn update_shooter(
  mut _commands: Commands,
  // assets: Res<BulletAssets>,
  input: Res<PlayerInput>,
  shooter_query: Query<(Entity, &Shooter)>,
  world: &mut World,
) {
  for (_entity, shooter) in shooter_query.iter() {
    if input.fire {
      // commands.entity(entity).with_children(|parent| {
      //   parent.spawn((
      //     PbrBundle {
      //       mesh: assets.bullet_mesh.clone(),
      //       material: assets.bullet_material.clone(),
      //       ..default()
      //     },
      //     Bullet { lifetime: 10.0 },
      //   ));
      // });
      shooter.weapon.left_click();
    }
    shooter.weapon.update(world);
  }
}

// TODO:ヒットスキャン
pub(super) fn _update_hit_scan() {}
