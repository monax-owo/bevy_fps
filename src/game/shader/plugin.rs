use bevy::prelude::*;

use super::shader::PostProcessMaterial;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_plugins(MaterialPlugin::<PostProcessMaterial>::default());
  }
}

// TODO:よくわからん
fn _init_camera(mut _images: ResMut<Assets<Image>>) {
  // カスタムのレンダーターゲット（テクスチャ）を作成
  // let render_texture = Image::new_fill(
  //   Extent3d {
  //     width: 1280,
  //     height: 720,
  //     depth_or_array_layers: 1,
  //   },
  //   TextureDimension::D2,
  //   &[0, 0, 0, 255],
  //   TextureFormat::Rgba8UnormSrgb,
  //   RenderAssetUsages::RENDER_WORLD,
  // );
  // images.add(render_texture);
}
