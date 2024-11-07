use bevy::prelude::*;

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
