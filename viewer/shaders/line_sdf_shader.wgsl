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
    let cam_forward = normalize(cam_target - cam_pos);
    let cam_right = normalize(cross(vec3<f32>(0., 1., 0.), cam_forward));
    let cam_up = normalize(cross(cam_forward, cam_right));

    let fov = 1.;
    let ray_dir = normalize(uv.x * cam_right + uv.y * cam_up + cam_forward * fov);

    return ray_dir;
}

fn sd_sphere(p: vec3<f32>, r: f32) -> f32 {
    return length(p) - r;
}

fn sd_capsule(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, r: f32) -> f32 {
    let pa = p - a;
    let ba = b - a;

    let h = clamp(dot(pa, ba) / dot(ba, ba), 0., 1.);

    return length(pa - ba * h) - r;
}

fn sdf(p: vec3<f32>) -> f32 {
    var t = 10.;

    for (var i = 0; i < number_of_lines(); i++) {
        let line = lines[i];
        let c = sd_capsule(p, line.a, line.b, 1.);

        if c < t {
            t = c;
        }
    }

    // let t = sd_capsule(p, vec3(-5., -5., 0.), vec3(5., 5., 0.), 1.);
    // let s = sd_sphere(p - vec3(1., 0., 0.), 1.);

    // if s < t {
    //     return s;
    // } else {
    //     return t;
    // }

    return t;
}

fn cast_ray(ray_origin: vec3<f32>, ray_dir: vec3<f32>) -> f32 {
    var t = 0.;

    for (var i = 0; i < 128; i++) {
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

    var col: vec3<f32>;
    if t == -1. {
        col = vec3<f32>(0.3, 0.36, 0.6) - (ray_dir.y * 0.3);
    } else {
        col = vec3(0.);
    }

    return col;
}

fn normalize_screen_coords(screen_coords: vec2<f32>, resolution: vec2<f32>) -> vec2<f32> {
    var result = 2. * (screen_coords / resolution - 0.5);
    result *= resolution.x / resolution.y;
    return result;
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(f32(camera_sdf.width), f32(camera_sdf.height));
    let uv = normalize_screen_coords(position.xy, resolution);

    let ray_dir = get_camera_ray_dir(uv, camera_sdf.eye, camera_sdf.look_at);

    let col = render(camera_sdf.eye, ray_dir);

    let color = vec4<f32>(col, 1.);

    return color;
}


fn number_of_lines() -> i32 {
    return i32(arrayLength(&lines));
}
