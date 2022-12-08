// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(3) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) tex_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) tex_index: u32,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.tex_index = model.tex_index;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_array: texture_2d_array<f32>;
@group(0) @binding(1)
var s: sampler;

struct FaceLightingUniform {
    positive: vec3<f32>,
    negative: vec3<f32>,
}
@group(2) @binding(0)
var<uniform> face_lighting: FaceLightingUniform;

let x_axis = vec3<f32>(1.0, 0.0, 0.0);
let y_axis = vec3<f32>(0.0, 1.0, 0.0);
let z_axis = vec3<f32>(0.0, 0.0, 1.0);

fn normal_shading(n: vec3<f32>) -> f32 {
    let ret = ((max(n, vec3(0.0)) * face_lighting.positive) + (-1.0 * min(n, vec3(0.0)) * face_lighting.negative)) * abs(n);
    return ret.x + ret.y + ret.z;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let face_brightness = normal_shading(in.normal);
    return textureSample(t_array, s, in.tex_coords, i32(in.tex_index)) * face_brightness;
}