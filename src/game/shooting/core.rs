use bevy::{color::palettes::css, prelude::*};

use crate::game::player::MovementInput;

#[derive(Resource, Debug, Default)]
pub(super) struct ShooterAssets {
  bullet_mesh: Handle<Mesh>,
  bullet_material: Handle<StandardMaterial>,
}

#[derive(Component, Reflect, Debug)]
pub(super) struct Shooter {
  firing_rate: f32,
}

#[derive(Component, Reflect, Debug)]
pub(super) struct Bullet {
  lifetime: f32,
}

pub(super) fn init_shooter(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let bullet_mesh = meshes.add(Sphere::new(0.4));
  let bullet_material = materials.add(Color::Srgba(css::BROWN));
  commands.insert_resource(ShooterAssets {
    bullet_mesh,
    bullet_material,
  });
}

pub(super) fn update_shooter(
  mut commands: Commands,
  assets: Res<ShooterAssets>,
  key: Res<MovementInput>,
  shooter_query: Query<(Entity, &Shooter)>,
) {
  for (entity, _shooter) in shooter_query.iter() {
    // TODO:fireなんてねえよ
    if key.fire {
      commands.entity(entity).with_children(|parent| {
        parent.spawn((
          PbrBundle {
            mesh: assets.bullet_mesh.clone(),
            material: assets.bullet_material.clone(),
            ..default()
          },
          Bullet { lifetime: 10.0 },
        ));
      });
    }
  }
}

// TODO:ヒットスキャン
pub(super) fn _update_hit_scan() {}
