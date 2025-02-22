// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}
struct Uniform {
    view_proj: mat4x4<f32>,
};
struct TextureOptions {
    size_x: f32,
    size_y: f32,
    size_z: f32,
    size_w: f32,
    texture_size_x: f32,
    texture_size_y: f32,
};
@group(0) @binding(0) 
var<uniform> uni: Uniform;
@group(0) @binding(3) 
var<uniform> texture_opt: TextureOptions;

struct CameraUniform {
    proj: mat4x4<f32>,
};
@group(1) @binding(0) 
var<uniform> cam_uni: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    if model.tex_coords.x == 0. {
        out.tex_coords.x = texture_opt.size_x / texture_opt.texture_size_x;
    } else {
        out.tex_coords.x = texture_opt.size_z / texture_opt.texture_size_x;
    }
    if model.tex_coords.y == 0. {
        out.tex_coords.y = 1-texture_opt.size_y / texture_opt.texture_size_y;
    } else {
        out.tex_coords.y = 1-texture_opt.size_w / texture_opt.texture_size_y;
    }
    out.clip_position = cam_uni.proj * (uni.view_proj * vec4<f32>(model.position , 1.0));
    return out;
}

// Fragment shader

@group(0) @binding(1)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(2)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}