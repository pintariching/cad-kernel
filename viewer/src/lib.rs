use glam::Vec3;
use line_sdf::LineStateSDF;
use std::sync::Arc;
use wgpu::BufferAddress;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent};
use winit::window::Window;

use kernel::TwoPointLine;

mod camera;
mod line;
mod line_sdf;
mod vertex;

use camera::{Camera, CameraState, CameraUniform};

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: PhysicalSize<u32>,
    pub window: Arc<Window>,
    pub config: wgpu::SurfaceConfiguration,
    pub line_state: LineStateSDF,
    pub camera_state: CameraState,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
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

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let lines = vec![TwoPointLine::new(
            Vec3::new(-0.5, 0., 0.),
            Vec3::new(0.5, 0., 0.),
        )];

        let camera = Camera {
            eye: Vec3::new(0., 1., 5.),
            target: Vec3::new(0., 0., 0.),
            up: Vec3::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.,
            znear: 0.1,
            zfar: 100.,
            width: size.width,
            height: size.height,
        };

        let camera_state = CameraState::new(camera, &device);
        // let line_state = LineState::new(lines, &device, &config, &camera_state);
        let line_sdf_state = LineStateSDF::new(lines, &device, &config, &camera_state);

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            size,
            window,
            config,
            line_state: line_sdf_state,
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

            self.camera_state.camera.width = new_size.width;
            self.camera_state.camera.height = new_size.height;

            // On macos the window needs to be redrawn manually after resizing
            self.window().request_redraw();
        }
    }

    pub fn input(&mut self, event: &KeyEvent, element_state: &ElementState) {
        self.camera_state
            .controller
            .process_events(event, element_state);
    }

    pub fn update(&mut self) {
        self.camera_state
            .controller
            .update_camera(&mut self.camera_state.camera);

        self.camera_state
            .uniform
            .update_view_proj(&self.camera_state.camera);

        self.queue.write_buffer(
            &self.camera_state.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_state.uniform]),
        );

        self.camera_state
            .sdf_uniform
            .update(&self.camera_state.camera);

        self.queue.write_buffer(
            &self.camera_state.sdf_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_state.sdf_uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&self.line_state.render_pipeline);
            rpass.set_bind_group(0, &self.camera_state.bind_group, &[]);
            rpass.set_bind_group(1, &self.line_state.bind_group, &[]);
            //rpass.set_vertex_buffer(0, self.line_state.line_buffer.slice(..));
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
