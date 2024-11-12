use bevy::prelude::*;

use crate::game::shooting::{
  bullet::projectile::{ProjectileBullet, ProjectileBulletAssets, ProjectileBulletGroup},
  FireEvent, Shooter,
};

#[derive(Component, Reflect, Debug, Default)]
pub struct TestGun {
  /// sec
  pub cool_time: f32,
  pub bullet_speed: f32,
}

const COOL_TIME: f32 = 0.4;

pub(super) fn update(
  mut commands: Commands,
  mut fire_event_reader: EventReader<FireEvent>,
  time: Res<Time>,
  group: Res<ProjectileBulletGroup>,
  assets: Res<ProjectileBulletAssets>,
  mut gun: Query<(&Shooter, &mut TestGun, &GlobalTransform)>,
) {
  for (_, mut gun, global_transform) in gun.iter_mut() {
    if gun.cool_time > 0.0 {
      gun.cool_time -= time.delta_seconds();
    }

    for _ in fire_event_reader.read() {
      if gun.cool_time <= 0.0 {
        // TODO:弾の発射処理はbulletの実装に移し、イベントで発火させる
        commands.entity(group.0).with_children(|parent| {
          parent.spawn((
            PbrBundle {
              mesh: assets.bullet_mesh.clone(),
              material: assets.bullet_material.clone(),
              transform: global_transform.compute_transform(),
              ..default()
            },
            ProjectileBullet {
              axis: global_transform.forward(),
              speed: gun.bullet_speed,
              lifetime: 6.0,
            },
          ));
        });

        gun.cool_time = COOL_TIME;
      }
    }
  }
}
