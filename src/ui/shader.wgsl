
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};
struct TransformUniform {
    scale: mat4x4<f32>,
    rotation: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> aspect_ratio: vec4<f32>;
@group(0) @binding(1)
var<uniform> transform: TransformUniform;


@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = transform.scale * transform.rotation * vec4<f32>(model.position, 1.0) * aspect_ratio;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}