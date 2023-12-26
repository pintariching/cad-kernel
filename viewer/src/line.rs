use std::f32::consts::PI;

use glam::{Mat3, Vec3};
use kernel::TwoPointLine;
use wgpu::include_wgsl;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::camera::{Camera, CameraState};
use crate::vertex::Vertex;

pub struct LineState {
    pub lines: Vec<TwoPointLine>,
    pub line_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
}

pub fn get_projected_vertices(lines: &Vec<TwoPointLine>, camera: &Camera) -> Vec<Vertex> {
    let projected_lines: Vec<TwoPointLine> = lines
        .iter()
        .map(|l| l.project_to_plane(camera.normal(), Vec3::new(0., 0., 0.)))
        .collect();

    let mut vertices = Vec::new();

    let width = 0.1;

    for line in &projected_lines {
        let line_dir = (line.b - line.a).normalize();
        let camera_normal = camera.normal();

        let rotation_matrix = Mat3::from_axis_angle(camera_normal, -PI / 2.); // PI / 2. = 90 degrees
        let offset = rotation_matrix * line_dir * (width / 2.);

        let tl = line.a + offset;
        let bl = line.a - offset;
        let tr = line.b + offset;
        let br = line.b - offset;

        let color = [1., 0., 0.];

        vertices.push(Vertex {
            position: bl.to_array(),
            color,
        });

        vertices.push(Vertex {
            position: tr.to_array(),
            color,
        });

        vertices.push(Vertex {
            position: tl.to_array(),
            color,
        });

        vertices.push(Vertex {
            position: tr.to_array(),
            color,
        });

        vertices.push(Vertex {
            position: bl.to_array(),
            color,
        });

        vertices.push(Vertex {
            position: br.to_array(),
            color,
        });
    }

    vertices
}

impl LineState {
    pub fn new(
        lines: Vec<TwoPointLine>,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_state: &CameraState,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/line_shader.wgsl"));

        //let vertices = get_projected_vertices(&lines, &camera_state.camera);

        let vertices = [
            Vertex {
                position: [0., 0., 0.],
                color: [0., 1., 0.],
            },
            Vertex {
                position: [1., 0., 0.],
                color: [0., 1., 0.],
            },
            Vertex {
                position: [0., 1., 0.],
                color: [0., 1., 0.],
            },
        ];

        let line_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Line Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_state.bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            lines,
            line_buffer,
            render_pipeline,
        }
    }
}
