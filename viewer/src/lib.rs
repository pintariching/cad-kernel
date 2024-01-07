use glam::Vec3;
use kernel::{Sketch, SketchArc, SketchElement, SketchPlane};
use std::sync::Arc;
use vertex::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::BufferAddress;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent};
use winit::window::Window;

mod camera;
mod sketch_state;
mod vertex;

use camera::{Camera, CameraState};
use sketch_state::SketchState;

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: PhysicalSize<u32>,
    pub window: Arc<Window>,
    pub config: wgpu::SurfaceConfiguration,
    pub camera_state: CameraState,
    pub sketch_state: SketchState,
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

        let camera = Camera {
            eye: Vec3::new(0., 0., 10.),
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

        let mut sketch_state = SketchState::new(0.1, &device, &config);
        sketch_state.add_sketch(Sketch {
            plane: SketchPlane::XY,
            elements: vec![SketchElement::Arc(SketchArc(kernel::arc::Arc {
                radius: 0.5,
                start: Vec3::new(0.5, 0., 0.),
                end: Vec3::new(0., 0.5, 0.),
                center: Vec3::new(0., 0., 0.),
                direction: kernel::arc::ArcDirection::CW,
            }))],
        });

        sketch_state.generate_lines();

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            size,
            window,
            config,
            camera_state,
            sketch_state,
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

        let mut line_verts = Vec::new();

        for line in &self.sketch_state.lines {
            let tpl = line.to_two_point_line();

            let a = tpl.a.0.to_array();
            let b = tpl.b.0.to_array();

            let v_a = Vertex {
                position: a,
                color: [0., 1., 0.],
            };

            let v_b = Vertex {
                position: b,
                color: [0., 1., 0.],
            };

            line_verts.push(v_a);
            line_verts.push(v_b);
        }

        self.sketch_state.tesselated_sketch_buffer =
            self.device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Tesselated Sketch Buffer"),
                contents: bytemuck::cast_slice(line_verts.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });
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

            rpass.set_pipeline(&self.sketch_state.render_pipeline);
            //rpass.set_bind_group(0, &self.camera_state.bind_group, &[]);
            rpass.set_vertex_buffer(0, self.sketch_state.tesselated_sketch_buffer.slice(..));
            rpass.draw(0..self.sketch_state.lines.len() as u32 * 2, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
