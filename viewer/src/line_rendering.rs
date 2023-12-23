use bytemuck::{Pod, Zeroable};
use kernel::Line;
use wgpu::include_wgsl;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RawTwoPointLine {
    a: [f32; 3],
    _padding: u32,
    b: [f32; 3],
    _padding2: u32,
}

impl RawTwoPointLine {
    fn from_line(line: &Line) -> Self {
        let l = line.to_two_point();

        Self {
            a: l.a.to_array(),
            _padding: 0,
            b: l.b.to_array(),
            _padding2: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct LineData {
    count: i32,
}

pub struct LineState {
    pub lines: Vec<Line>,
    pub line_data_buffer: wgpu::Buffer,
    pub line_data_bind_group: wgpu::BindGroup,
    pub line_buffer: wgpu::Buffer,
    pub line_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl LineState {
    pub fn new(
        lines: Vec<Line>,
        device: &wgpu::Device,
        vert_shader: &wgpu::VertexState,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/line_shader.wgsl"));

        let raw_lines = lines
            .iter()
            .map(|l| RawTwoPointLine::from_line(l))
            .collect::<Vec<RawTwoPointLine>>();

        let line_data = LineData {
            count: lines.len() as i32,
        };

        let line_data_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Line Data Buffer"),
            contents: bytemuck::cast_slice(&[line_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let line_data_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Line Data Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let line_data_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Line Data Bind Group"),
            layout: &line_data_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_data_buffer.as_entire_binding(),
            }],
        });

        let line_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Line Buffer"),
            contents: bytemuck::cast_slice(raw_lines.as_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let line_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Line Bind Group"),
            layout: &line_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &line_data_bind_group_layout,
                &line_bind_group_layout,
            ],
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
                topology: wgpu::PrimitiveTopology::LineList,
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
