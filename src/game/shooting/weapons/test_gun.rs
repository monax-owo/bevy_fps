use bevy::prelude::*;

use crate::game::shooting::{
  bullet::{Bullet, BulletAssets},
  FireEvent, Shooter,
};

#[derive(Component, Reflect, Debug, Default)]
pub struct TestGun {
  pub cool_time: f32,
}

const COOL_TIME: f32 = 2.0;

pub(super) fn update(
  mut commands: Commands,
  mut fire_event_reader: EventReader<FireEvent>,
  time: Res<Time>,
  assets: Res<BulletAssets>,
  mut gun: Query<(Entity, &Shooter, &mut TestGun, &Transform)>,
) {
  for (entity, _shooter, mut gun, transform) in gun.iter_mut() {
    if gun.cool_time > 0.0 {
      gun.cool_time -= time.delta_seconds();
    }

    for _ in fire_event_reader.read() {
      if gun.cool_time <= 0.0 {
        dbg!("fire");

        // TODO:スポーンする親を変え、初期位置は銃の位置にする
        commands.entity(entity).with_children(|parent| {
          parent.spawn((
            PbrBundle {
              mesh: assets.bullet_mesh.clone(),
              material: assets.bullet_material.clone(),
              ..default()
            },
            Bullet {
              axis: transform.forward().into(),
              speed: 4.0,
              lifetime: 6.0,
            },
          ));
        });

        gun.cool_time = COOL_TIME;
      }
    }
  }
}
