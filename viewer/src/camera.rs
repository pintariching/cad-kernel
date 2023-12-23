use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device,
    ShaderStages,
};

pub struct CameraState {
    pub camera: Camera,
    pub uniform: CameraUniform,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
}

impl CameraState {
    pub fn new(camera: Camera, device: &Device) -> Self {
        let uniform = CameraUniform::new_from_camera(&camera);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
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

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            camera,
            uniform,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }
}

pub struct Camera {
    pub eye: Vec3,
    pub width: f32,
    pub target: Vec3,
    pub height: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub eye: [f32; 3],
    pub width: f32,
    pub target: [f32; 3],
    pub height: f32,
}

impl CameraUniform {
    pub fn new_from_camera(camera: &Camera) -> Self {
        Self {
            eye: camera.eye.to_array(),
            width: camera.width,
            target: camera.target.to_array(),
            height: camera.height,
        }
    }
}
