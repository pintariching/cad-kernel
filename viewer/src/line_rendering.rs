use bytemuck::{Pod, Zeroable};
use kernel::{Line, ParametricLine};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    include_wgsl, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device,
    RenderPipeline, ShaderStages, SurfaceConfiguration, VertexState,
};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RawParametricLine {
    p: [f32; 3],
    _padding: u32,
    v: [f32; 3],
    _padding2: u32,
}

impl RawParametricLine {
    fn from_parametric_line(line: &ParametricLine) -> Self {
        Self {
            p: line.p.to_array(),
            _padding: 0,
            v: line.v.to_array(),
            _padding2: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct LineData {
    count: u32,
}

pub struct LineState {
    pub lines: Vec<Line>,
    pub line_data_buffer: Buffer,
    pub line_data_bind_group: BindGroup,
    pub line_buffer: Buffer,
    pub line_bind_group: BindGroup,
    pub render_pipeline: RenderPipeline,
}

impl LineState {
    pub fn new(
        lines: Vec<Line>,
        device: &Device,
        vert_shader: &VertexState,
        config: &SurfaceConfiguration,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/line_shader.wgsl"));

        let raw_lines = lines
            .iter()
            .map(|l| RawParametricLine::from_parametric_line(l.to_parametric()))
            .collect::<Vec<RawParametricLine>>();

        let line_data = LineData {
            count: lines.len() as u32,
        };

        let line_data_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Line Data Buffer"),
            contents: bytemuck::cast_slice(&[line_data]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let line_data_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Line Data Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    count: None,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let line_data_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Line Data Bind Group"),
            layout: &line_data_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: line_data_buffer.as_entire_binding(),
            }],
        });

        let line_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Line Buffer"),
            contents: bytemuck::cast_slice(raw_lines.as_slice()),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        let line_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Line Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                count: None,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        });

        let line_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Line Bind Group"),
            layout: &line_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&line_data_bind_group_layout, &line_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: vert_shader.clone(),
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
            line_data_buffer,
            line_data_bind_group,
            line_buffer,
            line_bind_group,
            render_pipeline,
        }
    }
}
