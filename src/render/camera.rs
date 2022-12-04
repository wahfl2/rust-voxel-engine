use nalgebra::{Vector3, Matrix4, Isometry3};
use wgpu::{Device, util::DeviceExt};

use super::util::math::perspective;

#[derive(Debug)]
pub struct Camera {
    pub transform: Isometry3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub buffer: Option<wgpu::Buffer>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Isometry3::identity(), 
            up: Vector3::new(0.0, 1.0, 0.0),
            aspect: 1.0,
            fov_y: 45.0, 
            z_near: 0.01, 
            z_far: 10000.0,
            buffer: None,
        }
    }
}

impl Camera {
    pub fn calculate_projection_matrix(&self) -> Matrix4<f32> {
        let proj = perspective(self.fov_y, self.aspect, self.z_near, self.z_far);
        proj * self.transform.to_matrix()
    }

    pub fn create_buffer(&mut self, device: &Device) -> &wgpu::Buffer {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[CameraUniform::from(&*self)]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        self.buffer = Some(buffer);
        &self.buffer.as_ref().unwrap()
    }

    pub fn get_bind_group_and_layout(&mut self, device: &Device) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_buffer = if let Some(buffer) = &self.buffer {
            buffer
        } else {
            self.create_buffer(device)
        };

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        (layout, bind_group)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }
}

impl From<&Camera> for CameraUniform {
    fn from(camera: &Camera) -> Self {
        Self {
            view_proj: camera.calculate_projection_matrix().into(),
        }
    }
}