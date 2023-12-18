struct Vertex {
    position: vec3<f32>,
}

@group(0) @binding(0)
var<storage, read> vertices: array<Vertex, 3>;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(in.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // for (var i = 0; i < 3; i++) {
    //     let v = vertices[i];

    //     if in.position.x > v.position.x {
    //         return vec4<f32>(1.);
    //     }
    // }

    discard;
}