use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::BindGroupDescriptor;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct CameraState {
    pub camera: Camera,
    pub controller: CameraController,
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub sdf_uniform: CameraUniformSDF,
    pub sdf_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl CameraState {
    pub fn new(camera: Camera, device: &wgpu::Device) -> Self {
        let mut uniform = CameraUniform::new();
        uniform.update_view_proj(&camera);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let sdf_uniform = CameraUniformSDF::from_camera(&camera);

        let sdf_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("SDF Camera Buffer"),
            contents: bytemuck::cast_slice(&[sdf_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                },
            ],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: sdf_buffer.as_entire_binding(),
                },
            ],
        });

        let controller = CameraController::new(0.005);

        Self {
            camera,
            controller,
            uniform,
            buffer,
            sdf_uniform,
            sdf_buffer,
            bind_group,
            bind_group_layout,
        }
    }
}

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);

        // let proj = Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);
        // proj * view

        view
    }

    pub fn normal(&self) -> Vec3 {
        (self.target - self.eye).normalize()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniformSDF {
    pub eye: [f32; 3],
    pub width: u32,
    pub target: [f32; 3],
    pub height: u32,
}

impl CameraUniformSDF {
    pub fn from_camera(camera: &Camera) -> Self {
        Self {
            eye: camera.eye.to_array(),
            target: camera.target.to_array(),
            width: camera.width,
            height: camera.height,
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.eye = camera.eye.to_array();
        self.target = camera.target.to_array();
        self.width = camera.width;
        self.height = camera.height;
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &KeyEvent, element_state: &ElementState) {
        let is_pressed = element_state.is_pressed();

        match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => {
                self.is_forward_pressed = is_pressed;
            }
            PhysicalKey::Code(KeyCode::KeyS) => {
                self.is_backward_pressed = is_pressed;
            }
            PhysicalKey::Code(KeyCode::KeyA) => {
                self.is_left_pressed = is_pressed;
            }
            PhysicalKey::Code(KeyCode::KeyD) => {
                self.is_right_pressed = is_pressed;
            }
            _ => (),
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_magn = forward.length();

        if self.is_forward_pressed && forward_magn > self.speed {
            camera.eye += forward_norm * self.speed;
        }

        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.length();

        if self.is_right_pressed {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
