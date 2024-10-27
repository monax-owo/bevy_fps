use bevy::prelude::*;

#[derive(Component)]
 pub  struct Crosshair {
   pub  color: Color,
}

impl Default for Crosshair {
  fn default() -> Self {
    Self {
      color: Color::srgb(1.0, 1.0, 1.0),
    }
  }
}

#[derive(Bundle)]
 pub  struct CrosshairBundle {
  crosshair: Crosshair,
  children: Children,
}

 pub  fn spawn_crosshair(mut commands: Commands) {
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
          // justify_content: JustifyContent::Center,
          // align_content: AlignContent::Center,
          // flex_wrap: FlexWrap::Wrap,
          ..default()
        },
        visibility: Visibility::Hidden,
        background_color: BackgroundColor(Color::srgb_u8(255, 255, 255)),
        ..default()
      });
      // .with_children(|parent| {
      //   for i in 1..=4 {
      //     parent.spawn(NodeBundle {
      //       style: Style {
      //         width: Val::Px(16.0),
      //         height: Val::Px(4.0),
      //         padding: UiRect::left(Val::Px(16.0)),
      //         position_type: PositionType::Absolute,
      //         ..default()
      //       },
      //       background_color: Color::srgb(1.0, 1.0, 1.0).into(),
      //       transform: Transform {
      //         rotation: Quat::from_rotation_z(f32::consts::FRAC_PI_2 * i as f32),
      //         ..default()
      //       },
      //       ..default()
      //     });
      //   }
      // });
    });
}

 pub  fn init_crosshair(
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
