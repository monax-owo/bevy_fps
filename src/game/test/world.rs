use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::shader::shader::PostProcessMaterial;

#[derive(Component)]
pub struct GenerateCollider(ComputedColliderShape);

pub(super) fn init_world(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<PostProcessMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.spawn(MaterialMeshBundle {
    mesh: meshes.add(Sphere::new(0.2)),
    material: materials.add(PostProcessMaterial {
      color: LinearRgba::WHITE,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 4.0, -2.0),
    ..default()
  });

  commands.spawn((
    SceneBundle {
      scene: asset_server.load("models/kusa.glb#Scene0"),
      ..default()
    },
    GenerateCollider(ComputedColliderShape::TriMesh),
  ));
}

// TODO:Colliderを自動生成
pub(super) fn generate_collider(
  mut commands: Commands,
  meshes: Res<Assets<Mesh>>,
  query: Query<(Entity, &Handle<Mesh>), (Added<Name>, Without<Collider>)>,
) {
  // &GenerateCollider,
  // , generate_collider
  for (entity, mesh) in query.iter() {
    println!("{}", entity.index());
    // commands.entity(entity).insert(
    //   (Collider::from_bevy_mesh(
    //     meshes.get(mesh).unwrap(),
    //     &ComputedColliderShape::ConvexDecomposition(VHACDParameters {}),
    //   ))
    //   .unwrap(),
    // );
  }
}
