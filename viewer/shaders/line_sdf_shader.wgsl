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

fn get_camera_ray_dir(uv: vec2<f32>, cam_pos: vec3<f32>, cam_target: vec3<f32>) -> vec3<f32> {
    let cam_forward = normalize(camera_sdf.look_at - camera_sdf.eye);
    let cam_right = normalize(cross(vec3<f32>(0., 1., 0.), cam_forward));
    let cam_up = normalize(cross(cam_forward, cam_right));

    let fov = 1.;
    let ray_dir = normalize(uv.x * cam_right + uv.y * cam_up + cam_forward * fov);

    return ray_dir;
}

fn sd_sphere(p: vec3<f32>, r: f32) -> f32 {
    return length(p) - r;
}

fn sd_box(p: vec3<f32>, b: vec3<f32>) -> f32 {
    let q = abs(p) - b;
    return length(max(q, 0.)) + min(max(q.x, max(q.y, q.z)), 0.);
}

fn sd_torus(p: vec3<f32>, t: vec2<f32>) -> f32 {
    let q = vec2(length(p.xz) - t.x, p.y);
}

fn sdf(p: vec3<f32>) -> f32 {

    //let t = sd_sphere(p - vec3<f32>(1., 0., 0.), 3.);
    let t = sd_box(p, vec3<f32>(1., 1., 1.));

    return t;
}

fn cast_ray(ray_origin: vec3<f32>, ray_dir: vec3<f32>) -> f32 {
    var t = 0.;

    for (var i = 0; i < 64; i++) {
        let res = sdf(ray_origin + ray_dir * t);

        if res < (0.0001 * t) {
            return t;
        } else {
            t += res;
        }
    }

    return -1.;
}

fn render(ray_origin: vec3<f32>, ray_dir: vec3<f32>) -> vec3<f32> {
    let t = cast_ray(ray_origin, ray_dir);

    let col = vec3<f32>(1. - t * 0.075);

    return col;
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(f32(camera_sdf.width), f32(camera_sdf.height));
    let uv = 2 * (position.xy / resolution - 0.5);

    let ray_dir = get_camera_ray_dir(uv, camera_sdf.eye, camera_sdf.look_at);

    let col = render(camera_sdf.eye, ray_dir);

    let color = vec4<f32>(col, 1.);

    return color;

    // let len = arrayLength(&lines);
    // for (var i = 0; i < number_of_lines(); i++) {
    //     let line = lines[i];

    //     let offset_x = (p.x / f32(camera_sdf.width) - 0.5) * 2. * camera_right;
    //     let offset_y = (p.y / f32(camera_sdf.height) - 0.5) * 2. * camera_up;

    //     let p = camera_position + offset_x + offset_y;

    //     if length(p.xy - line.a.xy) < 0.1 {
    //         return vec4<f32>(1.);
    //     }

    //     if length(p.xy - line.b.xy) < 0.1 {
    //         return vec4<f32>(1.);
    //     }
    // }

    // discard;
}


fn number_of_lines() -> i32 {
    return i32(arrayLength(&lines));
}
