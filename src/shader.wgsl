// Vertex shader


@group(0)
@binding(0)
var<uniform> view_matrix: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
     vertex: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = view_matrix * vertex.position;
    out.color = vertex.color;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}