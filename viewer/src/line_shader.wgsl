struct LineData {
    position: vec3<f32>,
}

struct Line {
    a: vec3<f32>,
    v: vec3<f32>
}

@group(0) @binding(0)
var<uniform> line_data: LineData;

@group(1) @binding(0)
var<storage, read> lines: array<Line>;

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    discard;
}
