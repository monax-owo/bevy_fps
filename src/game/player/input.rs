use bevy::prelude::*;

// TODO:キーボードとマウスの操作を分けるかPlayerInputに変える
#[derive(Resource, Debug, Default)]
pub struct PlayerInput {
  // キーボード
  pub forward: bool,
  pub left: bool,
  pub back: bool,
  pub right: bool,
  /// ジャンプ
  pub jump: bool,
  /// 走る
  pub dash: bool,
  /// 高速移動
  pub blink: bool,
  // マウス
  /// 発射
  pub fire: bool,
}

pub(super) fn update_input(
  keyboard: Res<ButtonInput<KeyCode>>,
  mouse: Res<ButtonInput<MouseButton>>,
  mut player_input: ResMut<PlayerInput>,
) {
  player_input.forward = keyboard.pressed(KeyCode::KeyW);
  player_input.left = keyboard.pressed(KeyCode::KeyA);
  player_input.back = keyboard.pressed(KeyCode::KeyS);
  player_input.right = keyboard.pressed(KeyCode::KeyD);

  player_input.jump = keyboard.pressed(KeyCode::Space);
  player_input.dash = keyboard.pressed(KeyCode::KeyV);
  player_input.blink = keyboard.pressed(KeyCode::KeyQ);

  // TODO:切り離す
  player_input.fire = mouse.pressed(MouseButton::Left);
}
