use bevy::{
  prelude::*,
  window::{CursorGrabMode, PrimaryWindow},
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
  #[default]
  PauseMenu,
  InGame,
  // TODO:メインメニュー
  // MainMenu,
}

pub(super) fn update_menu(
  state: Res<State<GameState>>,
  mut next_state: ResMut<NextState<GameState>>,
  key: Res<ButtonInput<KeyCode>>,
) {
  if key.just_released(KeyCode::Escape) {
    println!("released");
    next_state.set(match state.get() {
      GameState::PauseMenu => GameState::InGame,
      GameState::InGame => GameState::PauseMenu,
    });
  }
}

pub(super) fn on_enter(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
  if let Ok(window) = window_query.get_single_mut() {
    util_cursor_lock(window, true);
  }
}

pub(super) fn on_exit(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
  if let Ok(window) = window_query.get_single_mut() {
    util_cursor_lock(window, false);
  }
}

pub(super) fn util_cursor_lock(mut window: Mut<'_, Window>, val: bool) {
  window.cursor.grab_mode = if val {
    CursorGrabMode::Locked
  } else {
    CursorGrabMode::None
  };
  window.cursor.visible = !val;
}
