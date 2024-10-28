use bevy::prelude::*;

use super::player::Player;

// TODO:プレイヤーを動かせるようにする
pub(super) fn update_movement(
  key: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut query: Query<(&Player, &mut Transform)>,
) {
  if let Ok((player, mut player_transform)) = query.get_single_mut() {
    let mut direction = Vec3::ZERO;

    if key.pressed(KeyCode::KeyW) {
      direction += *player_transform.forward();
    }

    if key.pressed(KeyCode::KeyA) {
      direction += *player_transform.left();
    }

    if key.pressed(KeyCode::KeyS) {
      direction += *player_transform.back();
    }

    if key.pressed(KeyCode::KeyD) {
      direction += *player_transform.right();
    }

    player_transform.translation +=
      direction.clamp_length(0.0, 1.0).normalize_or_zero() * player.speed * time.delta_seconds();
  }
}
