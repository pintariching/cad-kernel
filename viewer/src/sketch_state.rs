use kernel::line::Line;
use kernel::Sketch;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{include_wgsl, BufferAddress};

use crate::vertex::Vertex;

pub struct SketchState {
    pub sketches: Vec<Sketch>,
    pub line_width: f32,
    pub lines: Vec<Line>,
    pub tesselated_sketch_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl SketchState {
    pub fn new(
        line_width: f32,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/sketch_shader.wgsl"));

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sketch Vertex Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        //     label: Some("Test Vertex Buffer"),
        //     contents: bytemuck::cast_slice(&[
        //         Vertex {
        //             position: [-0.5, 0., 0.],
        //             color: [0., 1., 0.],
        //         },
        //         Vertex {
        //             position: [-0.5, 0.5, 0.],
        //             color: [0., 1., 0.],
        //         },
        //         Vertex {
        //             position: [0., 0., 0.],
        //             color: [0., 1., 0.],
        //         },
        //         Vertex {
        //             position: [0., 0.5, 0.],
        //             color: [0., 1., 0.],
        //         },
        //         Vertex {
        //             position: [0.5, 0., 0.],
        //             color: [0., 1., 0.],
        //         },
        //         Vertex {
        //             position: [0.5, 0.5, 0.],
        //             color: [0., 1., 0.],
        //         },
        //     ]),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sketch Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sketch Render Pipeline"),
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
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
            sketches: Vec::new(),
            line_width,
            lines: Vec::new(),
            tesselated_sketch_buffer: vertex_buffer,
            render_pipeline,
        }
    }

    pub fn add_sketch(&mut self, sketch: Sketch) {
        self.sketches.push(sketch);
    }

    pub fn update_line_width(&mut self, line_width: f32) {
        self.line_width = line_width;
    }

    pub fn generate_lines(&mut self) {
        self.lines = Vec::new();

        for sketch in &self.sketches {
            let mut l = sketch.to_lines();

            self.lines.append(&mut l);
        }
    }
}
