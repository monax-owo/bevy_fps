use bevy::prelude::*;

use crate::game::player::input::PlayerInput;

pub(super) trait Weapon {
  fn left_click(&self) {}
  fn right_click(&self) {}
  fn firing_rate(&self) -> f32 {
    1.0
  }
  fn update(&self, world: &mut World) {
    let _ = world;
  }
}

// todo:ReflectとDebugを実装したい
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
    }
  }
}

// TODO:ヒットスキャン
pub(super) fn _update_hit_scan() {}
