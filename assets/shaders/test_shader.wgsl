#import bevy_pbr::mesh_view_bindings::globals
#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4(abs(vec3(abs(sin(globals.time))) - material_color.rgb), material_color.a);
}