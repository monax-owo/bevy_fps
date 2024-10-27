use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;

#[derive(Clone, Asset, AsBindGroup, TypePath)]
pub struct PostProcessMaterial {
  #[uniform(0)]
  pub color: LinearRgba,
  #[texture(1)]
  #[sampler(2)]
  pub color_texture: Option<Handle<Image>>,
}

impl Default for PostProcessMaterial {
  fn default() -> Self {
    Self {
      color: Default::default(),
      color_texture: Default::default(),
    }
  }
}

// TODO:<https://bevyengine.org/examples/2d-rendering/pixel-grid-snap/>これでよくない？試します
impl Material for PostProcessMaterial {
  fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
    "shaders/test_shader.wgsl".into()
  }
}
