use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use camera::{Camera, CameraState};
use glam::Vec3;
use kernel::{Line, TwoPointLine};
use line_rendering::LineState;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    include_wgsl, vertex_attr_array, Backends, Buffer, BufferAddress, BufferUsages, Color,
    CommandEncoderDescriptor, Device, Instance, InstanceDescriptor, LoadOp, Operations, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, StoreOp, Surface, SurfaceConfiguration,
    SurfaceError, TextureUsages, TextureViewDescriptor, VertexBufferLayout, VertexState,
    VertexStepMode,
};
use winit::dpi::PhysicalSize;
use winit::event::KeyEvent;
use winit::window::Window;

mod camera;
mod line_rendering;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RawVertex {
    position: [f32; 3],
}

impl RawVertex {
    fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<RawVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float32x3],
        }
    }
}

const CLIP_VERTICES: &[RawVertex] = &[
    RawVertex {
        position: [-1., 1., 0.],
    },
    RawVertex {
        position: [-1., -3., 0.],
    },
    RawVertex {
        position: [3., 1., 0.],
    },
];

pub struct State<'a> {
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub size: PhysicalSize<u32>,
    pub window: Arc<Window>,
    pub config: SurfaceConfiguration,
    pub line_state: LineState,
    pub clip_vertex_buffer: Buffer,
    pub camera_state: CameraState,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // The surface needs to live as long as the window that created it.
        // State owns the window, so this should be safe.
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let vert_shader =
            device.create_shader_module(include_wgsl!("../shaders/base_vert_shader.wgsl"));
        let vert_shader_state = VertexState {
            module: &vert_shader,
            entry_point: "vs_main",
            buffers: &[RawVertex::desc()],
        };

        let clip_vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Clip Vertex Buffer"),
            contents: bytemuck::cast_slice(CLIP_VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let lines = vec![Line::TwoPoint(TwoPointLine::new(
            Vec3::new(-1., -2., 3.),
            Vec3::new(1., 1., 1.),
        ))];

        let camera = Camera {
            eye: Vec3::new(0., 0., 10.),
            target: Vec3::new(0., 0., 0.),
            width: size.width as f32,
            height: size.height as f32,
        };

        let camera_normal = (camera.target - camera.eye).normalize();
        let projected_lines = lines
            .iter()
            .map(|l| {
                let l = l.to_two_point();

                let a_e = l.a - camera.eye;
                let b_e = l.b - camera.eye;

                let w_a = a_e.project_onto(camera_normal);
                let w_b = b_e.project_onto(camera_normal);

                dbg!(&w_a);
                dbg!(&w_b);

                Line::TwoPoint(TwoPointLine {
                    a: l.a - w_a,
                    b: l.b - w_b,
                })
            })
            .collect();

        dbg!(&projected_lines);

        let camera_state = CameraState::new(camera, &device);
        let line_state = LineState::new(
            projected_lines,
            &device,
            &vert_shader_state,
            &config,
            &camera_state.bind_group_layout,
        );

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            size,
            window,
            config,
            line_state,
            clip_vertex_buffer,
            camera_state,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.height = new_size.height;
            self.config.width = new_size.width;
            self.surface.configure(&self.device, &self.config);

            // On macos the window needs to be redrawn manually after resizing
            self.window().request_redraw();
        }
    }

    pub fn input(&mut self, event: &KeyEvent) {}

    pub fn update(&mut self) {
        self.camera_state.camera.eye.x += 1.;
        self.queue.write_buffer(
            &self.camera_state.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_state.uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let frame = self.surface.get_current_texture()?;

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&self.line_state.render_pipeline);
            rpass.set_bind_group(0, &self.camera_state.bind_group, &[]);
            rpass.set_bind_group(1, &self.line_state.line_data_bind_group, &[]);
            rpass.set_bind_group(2, &self.line_state.line_bind_group, &[]);
            rpass.set_vertex_buffer(0, self.clip_vertex_buffer.slice(..));
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
