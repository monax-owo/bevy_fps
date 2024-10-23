use bevy::prelude::*;

#[derive(Component)]
pub struct Crosshair {
  pub color: Color,
}

impl Default for Crosshair {
  fn default() -> Self {
    Self {
      color: Color::srgb(1.0, 1.0, 1.0),
    }
  }
}

#[derive(Bundle)]
pub struct CrosshairBundle {
  crosshair: Crosshair,
  children: Children,
}

pub fn init_crosshair(mut commands: Commands) {
  commands
    .spawn(NodeBundle {
      style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        flex_wrap: FlexWrap::Wrap,
        display: Display::Flex,
        ..default()
      },
      ..default()
    })
    // .push_children(&[]);
    .with_children(|parent| {
      parent.spawn((
        Crosshair {
          color: Color::srgb_u8(255, 255, 255),
        },
        NodeBundle {
          style: Style {
            width: Val::Px(16.0),
            height: Val::Px(4.0),
            display: Display::Block,
            ..default()
          },
          background_color: Color::srgba(1.0, 1.0, 1.0, 0.4).into(),
          ..default()
        },
      ));
    });
}
