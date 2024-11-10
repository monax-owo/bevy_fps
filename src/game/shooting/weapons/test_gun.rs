use bevy::prelude::*;

use crate::game::shooting::{
  bullet::projectile::{ProjectileBullet, ProjectileBulletAssets},
  FireEvent, Shooter,
};

#[derive(Component, Reflect, Debug, Default)]
pub struct TestGun {
  pub cool_time: f32,
}

const COOL_TIME: f32 = 0.4;

pub(super) fn update(
  mut commands: Commands,
  mut fire_event_reader: EventReader<FireEvent>,
  time: Res<Time>,
  assets: Res<ProjectileBulletAssets>,
  mut gun: Query<(&Shooter, &mut TestGun, &GlobalTransform)>,
) {
  for (_, mut gun, global_transform) in gun.iter_mut() {
    if gun.cool_time > 0.0 {
      gun.cool_time -= time.delta_seconds();
    }

    for _ in fire_event_reader.read() {
      if gun.cool_time <= 0.0 {
        commands.spawn((
          PbrBundle {
            mesh: assets.bullet_mesh.clone(),
            material: assets.bullet_material.clone(),
            transform: global_transform.compute_transform(),
            ..default()
          },
          ProjectileBullet {
            axis: global_transform.forward(),
            speed: 100.0,
            lifetime: 6.0,
          },
        ));

        gun.cool_time = COOL_TIME;
      }
    }
  }
}
