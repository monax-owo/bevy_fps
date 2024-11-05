use bevy::prelude::*;

// TODO:画面の状態を管理する
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
  #[default]
  MainMenu,
  InGame,
  // TODO:ポーズ画面
  // PauseMenu,
}

pub(super) fn update_menu(
  state: Res<State<GameState>>,
  mut next_state: ResMut<NextState<GameState>>,
  key: Res<ButtonInput<KeyCode>>,
) {
  if key.just_released(KeyCode::Escape) {
    println!("released");
    next_state.set(match state.get() {
      GameState::MainMenu => GameState::InGame,
      GameState::InGame => GameState::MainMenu,
    });
  }
}
