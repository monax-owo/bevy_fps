use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(super) struct PauseMenu;

// TODO:メインメニューからInGameに移ったらアセットをロードする
pub(super) fn spawn_ui(mut commands: Commands) {
  commands
    .spawn((
      Name::new("PauseMenu"),
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          ..default()
        },
        background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.4)),
        ..default()
      },
      PauseMenu,
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

pub(super) fn despawn_ui(mut commands: Commands, ui_query: Query<(Entity, &PauseMenu)>) {
  if let Ok((entity, _)) = ui_query.get_single() {
    commands.entity(entity).despawn_recursive();
  }
}
