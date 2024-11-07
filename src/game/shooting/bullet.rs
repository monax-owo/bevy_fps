use bevy::{color::palettes::css, prelude::*};

#[derive(Component, Reflect, Debug)]
pub(super) struct Bullet {
  pub lifetime: f32,
}

#[derive(Resource, Debug, Default)]
pub(super) struct BulletAssets {
  pub bullet_mesh: Handle<Mesh>,
  pub bullet_material: Handle<StandardMaterial>,
}

pub(super) fn init_bullet(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let bullet_mesh = meshes.add(Sphere::new(0.4));
  let bullet_material = materials.add(Color::Srgba(css::BROWN));
  commands.insert_resource(BulletAssets {
    bullet_mesh,
    bullet_material,
  });
}
