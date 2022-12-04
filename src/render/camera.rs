use std::f32::consts::FRAC_PI_2;

use nalgebra::{Vector3, Matrix4, Isometry3, Rotation3, Point3};
use wgpu::{Device, util::DeviceExt};
use winit::event::DeviceEvent;

use crate::util::constants::DEFAULT_MOUSE_SENS;

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

#[derive(Debug, Clone)]
pub struct CameraController {
    pub position: Point3<f32>,
    pub prev_yaw: f32,
    pub yaw: f32,
    pub prev_pitch: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            prev_yaw: 0.0,
            yaw: 0.0, 
            prev_pitch: 0.0,
            pitch: 0.0, 
            sensitivity: 1.0,
        }
    }
}

impl CameraController {
    pub fn new(position: Point3<f32>, sensitivity: f32) -> Self {
        Self {
            position,
            prev_yaw: 0.0,
            yaw: 0.0, 
            prev_pitch: 0.0,
            pitch: 0.0, 
            sensitivity,
        }
    }

    pub fn process_device_event(&mut self, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            let (dx, dy) = (delta.0 as f32, delta.1 as f32);
            self.yaw += dx * DEFAULT_MOUSE_SENS * self.sensitivity;
            self.pitch += dy * DEFAULT_MOUSE_SENS * self.sensitivity;
            
            self.pitch = self.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
        }
    }

    pub fn get_transform(&mut self) -> Isometry3<f32> {
        let mut ret = Isometry3::from_parts(
            self.position.into(), 
            Rotation3::identity().into(),
        );
        ret.append_rotation_mut(&Rotation3::from_axis_angle(&Vector3::y_axis(), self.yaw).into());
        ret.append_rotation_mut(&Rotation3::from_axis_angle(&Vector3::x_axis(), self.pitch).into());
        ret
    }
}