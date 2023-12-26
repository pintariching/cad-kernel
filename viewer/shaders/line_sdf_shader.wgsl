struct CameraUniform {
    view_proj: mat4x4<f32>
}

struct CameraUniformSDF {
    eye: vec3<f32>,
    width: u32,
    look_at: vec3<f32>,
    height: u32,
}

struct Line {
    a: vec3<f32>,
    b: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(0) @binding(1)
var<uniform> camera_sdf: CameraUniformSDF;

@group(1) @binding(0)
var<storage, read> lines: array<Line>;

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let len = arrayLength(&lines);

    for (var i = 0; i < number_of_lines(); i++) {
        let line = lines[i];
        let p = position.xy;

        let offset_x = p.x / f32(camera_sdf.width) - 0.5;
        let offset_y = p.y / f32(camera_sdf.height) - 0.5;

        let p_x = camera_sdf.eye.xy + vec2<f32>(offset_x, offset_y);

        if length(p_x - line.a.xy) < 0.1 {
            return vec4<f32>(1.);
        }
    }

    discard;
}

fn number_of_lines() -> i32 {
    return i32(arrayLength(&lines));
}