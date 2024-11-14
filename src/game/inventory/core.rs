use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
pub struct PlayerInventory {
  /// current_itemのモデルを表示させるエンティティ
  /// `None`の場合はこのコンポーネントを持っているエンティティを指す
  pub model_applier: Option<Entity>,
}

impl PlayerInventory {
  pub fn new(model_applier: Entity) -> Self {
    Self {
      model_applier: Some(model_applier),
    }
  }
}

pub(super) fn update_model() {
  // todo!();
  // TODO: current_itemのモデルを表示させる
  //   PbrBundle {
  //   mesh: meshes.add(Cuboid::new(0.2, 0.4, 0.8)),
  //   material: materials.add(Color::Srgba(css::DARK_GRAY)),
  //   ..default()
  // },
}
