use bevy::prelude::*;

#[derive(Component, Debug)]
pub(super) struct MainMenu;

// TODO:メインメニューからInGameに移ったらアセットをロードする
pub(super) fn spawn_ui(mut commands: Commands) {
  commands
    .spawn((
      Name::new("MainMenu"),
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          ..default()
        },
        background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.4)),
        ..default()
      },
      MainMenu,
    ))
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "PauseMenu",
        TextStyle {
          color: Color::BLACK,
          ..default()
        },
      ));
    });
}

pub(super) fn despawn_ui(mut commands: Commands, ui_query: Query<(Entity, &MainMenu)>) {
  if let Ok((entity, _)) = ui_query.get_single() {
    commands.entity(entity).despawn_recursive();
  }
}
