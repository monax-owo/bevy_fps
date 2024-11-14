use bevy::prelude::*;

use crate::game::shooting::{
  bullet::{ProjectileBulletAssets, ProjectileBulletBundle, ProjectileBulletGroup},
  FireEvent,
};

#[derive(Component, Reflect, Debug)]
pub struct TestGun {
  /// sec
  pub cool_time: f32,
  pub bullet_speed: f32,
  pub bullet_lifetime: f32,
}

impl TestGun {
  const COOL_TIME: f32 = 0.4;
}

pub(super) fn update(
  mut commands: Commands,
  mut fire_event_reader: EventReader<FireEvent>,
  time: Res<Time>,
  group: Res<ProjectileBulletGroup>,
  assets: Res<ProjectileBulletAssets>,
  mut gun: Query<(&mut TestGun, &GlobalTransform)>,
) {
  for (mut gun, global_transform) in gun.iter_mut() {
    if gun.cool_time > 0.0 {
      gun.cool_time -= time.delta_seconds();
    }

    for _ in fire_event_reader.read() {
      if gun.cool_time <= 0.0 {
        // TODO:弾の発射処理はbulletの実装に移し、イベントで発火させる
        commands.entity(group.0).with_children(|parent| {
          parent.spawn(ProjectileBulletBundle::new(
            assets.bullet_mesh.clone(),
            assets.bullet_material.clone(),
            global_transform.compute_transform(),
            gun.bullet_speed,
            gun.bullet_lifetime,
          ));
        });

        gun.cool_time = TestGun::COOL_TIME;
      }
    }
  }
}
