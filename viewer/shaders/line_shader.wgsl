struct CameraUniform {
    eye: vec3<f32>,
    width: f32,
    look_at: vec3<f32>,
    height: f32,
}

struct LineData {
    count: i32,
}

struct TwoPointLine {
    a: vec3<f32>,
    b: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> line_data: LineData;

@group(2) @binding(0)
var<storage, read> lines: array<TwoPointLine>;

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    for (var i = 0; i < 2; i++) {
        let line = lines[i];

        let p_x = ((2 * position.x) / camera.width) - 0.5;
        let p_y = ((2 * position.y) / camera.height) - 0.5;
        let p_offset = vec3<f32>(p_x, p_y, 0.);

        let p = camera.eye + p_offset;

        if length(line.a.xy - p.xy) < 0.05 {
            return vec4<f32>(1.);
        }

        if length(line.b.xy - p.xy) < 0.05 {
            return vec4<f32>(1.);
        }
    }

    discard;
}
