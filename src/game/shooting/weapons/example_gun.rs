use bevy::prelude::*;

use crate::game::{
  inventory::CurrentWeapon,
  shooting::{
    bullet::{ProjectileBulletAssets, ProjectileBulletBundle, ProjectileBulletGroup},
    FireEvent,
  },
};

#[derive(Component, Reflect, Debug)]
pub struct ExampleGun {
  pub cool_time: Timer,
  pub bullet_speed: f32,
  pub bullet_lifetime: f32,
}

pub(super) fn update(
  mut commands: Commands,
  mut fire_event_reader: EventReader<FireEvent>,
  time: Res<Time>,
  group: Res<ProjectileBulletGroup>,
  assets: Res<ProjectileBulletAssets>,
  mut gun: Query<(&mut ExampleGun, &GlobalTransform), With<CurrentWeapon>>,
) {
  for (mut gun, global_transform) in gun.iter_mut() {
    for _ in fire_event_reader.read() {
      if gun.cool_time.finished() {
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

        gun.cool_time.reset();
      }
    }

    gun.cool_time.tick(time.delta());
  }
}
