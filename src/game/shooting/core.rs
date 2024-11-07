use std::fmt::Debug;

use bevy::prelude::*;

use crate::game::player::MovementInput;

use super::bullet::{Bullet, BulletAssets};

// TODO:銃の機能を実装するトレイトを作る
pub(super) trait Weapon {
  fn left_click(&self);
  fn right_click(&self);
  fn firing_rate(&self) -> f32;
}

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

pub(super) fn init_shooter(mut _commands: Commands) {}

pub(super) fn update_shooter(
  mut _commands: Commands,
  // assets: Res<BulletAssets>,
  key: Res<MovementInput>,
  shooter_query: Query<(Entity, &Shooter)>,
) {
  for (entity, shooter) in shooter_query.iter() {
    // TODO:fireなんてねえよ
    if key.fire {
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
  }
}

// TODO:ヒットスキャン
pub(super) fn _update_hit_scan() {}
