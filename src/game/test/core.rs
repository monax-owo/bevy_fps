use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub(super) struct TestTag(pub String);

// 使い終わったら消す
pub(super) fn init_tester(mut commands: Commands, asset_server: Res<AssetServer>) {
  let scene = asset_server.load("models/test_gun2.glb#Scene0");

  commands.spawn((
    SceneBundle {
      scene,
      transform: Transform::from_scale(Vec3::splat(4.0)),
      ..default()
    },
    AnimationPlayer::default(),
  ));
}

// TODO:Animation Graphってのがいる？わからないので寝る。
pub(super) fn update_tester(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut graphs: ResMut<Assets<AnimationGraph>>,
  mut q: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
  // TODO:iter要らないマ？
  for (entity, mut player) in &mut q {
    let mut transitions = AnimationTransitions::new();
    let animation =
      asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/test_gun2.glb"));

    let (graph, animation) = AnimationGraph::from_clip(animation);

    transitions
      .play(&mut player, animation, Duration::ZERO)
      .repeat();

    commands.entity(entity).insert(graphs.add(graph));
  }
}
