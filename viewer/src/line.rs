use glam::Vec3;
use kernel::line::Line;
use kernel::Plane;
use wgpu::include_wgsl;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::camera::CameraState;
use crate::vertex::Vertex;

pub struct LineState {
    pub lines: Vec<Line>,
    pub line_projection_plane: Plane,
    pub line_width: f32,
    pub line_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl LineState {
    pub fn new(
        lines: Vec<Line>,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_state: &CameraState,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/line_shader.wgsl"));

        let camera_normal = camera_state.camera.normal();
        let line_plane = Plane::new(camera_normal, Vec3::ZERO);
        let line_width = 0.1;

        let vertices = generate_vertices(&lines, &line_plane, line_width);

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
                cull_mode: None,
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
            line_projection_plane: line_plane,
            line_width,
            render_pipeline,
        }
    }

    pub fn update_line_projection_plane(&mut self, camera_state: &CameraState) {
        self.line_projection_plane.normal = camera_state.camera.normal();
    }

    pub fn update_line_width(&mut self, camera_state: &CameraState) {
        let cam_dist = camera_state.camera.eye.length();
        self.line_width = 0.01 * cam_dist;
    }

    pub fn generate_quad_vertices(&self) -> Vec<Vertex> {
        generate_vertices(&self.lines, &self.line_projection_plane, self.line_width)
    }
}

fn generate_vertices(lines: &Vec<Line>, projection_plane: &Plane, line_width: f32) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for line in lines {
        let verts = line.generate_projected_quad(&projection_plane, line_width);

        for v in verts {
            vertices.push(Vertex {
                position: v.to_array(),
                color: [0., 1., 0.],
            })
        }
    }

    vertices
}
