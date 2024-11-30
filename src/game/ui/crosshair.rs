use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(super) struct Crosshair {
  /// クロスヘアの色
  pub(super) color: Color,
}

impl Default for Crosshair {
  fn default() -> Self {
    Self {
      color: Color::srgb(1.0, 1.0, 1.0),
    }
  }
}

#[derive(Bundle)]
pub(super) struct CrosshairBundle {
  crosshair: Crosshair,
  children: Children,
}

pub(super) fn spawn_crosshair(mut commands: Commands) {
  commands
    .spawn((
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_content: AlignContent::Center,
          flex_wrap: FlexWrap::Wrap,
          ..default()
        },
        ..default()
      },
      Crosshair {
        color: Color::srgb_u8(255, 255, 255),
      },
    ))
    .with_children(|parent| {
      parent.spawn(NodeBundle {
        style: Style {
          width: Val::Px(4.0),
          height: Val::Px(4.0),
          ..default()
        },
        background_color: BackgroundColor(Color::srgb_u8(255, 255, 255)),
        ..default()
      });
    });
}

pub(super) fn init_crosshair(
  crosshair: Query<&Crosshair, With<Node>>,
  mut _bg: Query<&BackgroundColor, (With<Node>, Without<Crosshair>)>,
) {
  dbg!(crosshair.single().color);
  // #[allow(unused_assignments, unused_variables)]
  // if let (Ok(crosshair), Ok(mut bg)) = (crosshair.get_single(), bg.get_single_mut()) {
  //   bg = &BackgroundColor(crosshair.color);
  //   dbg(1);
  // }
}
