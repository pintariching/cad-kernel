use bytemuck::{Pod, Zeroable};
use kernel::line::TwoPointLine;
use wgpu::include_wgsl;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::camera::CameraState;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RawTwoPointLine {
    a: [f32; 3],
    _padding: u32,
    b: [f32; 3],
    _padding2: u32,
}

impl RawTwoPointLine {
    pub fn from_two_point_line(line: &TwoPointLine) -> Self {
        Self {
            a: line.a.to_array(),
            b: line.b.to_array(),
            _padding: 0,
            _padding2: 0,
        }
    }
}

pub struct LineStateSDF {
    pub lines: Vec<TwoPointLine>,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl LineStateSDF {
    pub fn new(
        lines: Vec<TwoPointLine>,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_state: &CameraState,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/line_sdf_shader.wgsl"));
        let vert_shader =
            device.create_shader_module(include_wgsl!("../shaders/base_vert_shader.wgsl"));

        let raw_lines: Vec<RawTwoPointLine> = lines
            .iter()
            .map(RawTwoPointLine::from_two_point_line)
            .collect();

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Line Buffer"),
            contents: bytemuck::cast_slice(raw_lines.as_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Line Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Line Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_state.bind_group_layout, &bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "vs_main",
                buffers: &[],
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
            lines,
            buffer,
            bind_group,
            bind_group_layout,
            render_pipeline,
        }
    }
}
