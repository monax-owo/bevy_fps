use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerInput {
  // キーボード
  pub forward: KeyCode,
  pub left: KeyCode,
  pub back: KeyCode,
  pub right: KeyCode,
  /// ジャンプ
  pub jump: KeyCode,
  /// 走る
  pub dash: KeyCode,
  /// 高速移動
  pub blink: KeyCode,
  // マウス
  /// 発射
  pub fire: MouseButton,
}

impl Default for PlayerInput {
  fn default() -> Self {
    Self {
      forward: KeyCode::KeyW,
      left: KeyCode::KeyA,
      back: KeyCode::KeyS,
      right: KeyCode::KeyD,
      jump: KeyCode::Space,
      dash: KeyCode::KeyV,
      blink: KeyCode::KeyQ,
      fire: MouseButton::Left,
    }
  }
}
