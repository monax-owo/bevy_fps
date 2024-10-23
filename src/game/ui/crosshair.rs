use bevy::prelude::*;

#[derive(Component)]
pub struct Crosshair {
  color: Color,
}

pub fn init_crosshair(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn(NodeBundle {
      style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
      },
      ..default()
    })
    // .push_children(&[]);
    .with_children(|parent| {
      parent
        .spawn((Crosshair {
          color: Color::srgb_u8(255, 255, 255),
        },))
        .with_children(|parent| {
          for i in 1..=4 {
            parent.spawn(MaterialMeshBundle {
              mesh: meshes.add(Rectangle::new(1.0, 4.0)),
              material: materials.add(Color::srgb_u8(255, 255, 255)),
              ..default()
            });
          }
        });
    });
}
