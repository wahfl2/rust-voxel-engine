use bytemuck::bytes_of;
use nalgebra::Vector3;
use wgpu::{Device, util::DeviceExt};

pub struct FaceLighting {
    pub positive: Vector3<f32>,
    pub negative: Vector3<f32>,
    pub buffer: Option<wgpu::Buffer>,
    pub changed: bool,
}

impl Default for FaceLighting {
    fn default() -> Self {
        Self { 
            positive: Vector3::new(1.0, 1.0, 1.0), 
            negative: Vector3::new(1.0, 1.0, 1.0),
            buffer: None,
            changed: true,
        }
    }
}

impl FaceLighting {
    pub fn new(up: f32, down: f32, x_axis: f32, z_axis: f32) -> Self {
        Self {
            positive: Vector3::new(x_axis, up, z_axis),
            negative: Vector3::new(x_axis, down, z_axis),
            buffer: None,
            changed: true,
        }
    }

    pub fn create_buffer(&mut self, device: &Device) -> &wgpu::Buffer {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Face Lighting Buffer"),
                contents: bytemuck::cast_slice(&[FaceLightingUniform::from(&*self)]),
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
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("face_lighting_bind_group_layout"),
        });

        let buf = if let Some(buffer) = &self.buffer {
            buffer
        } else {
            self.create_buffer(device)
        };

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buf.as_entire_binding(),
                }
            ],
            label: Some("face_lighting_bind_group"),
        });

        (layout, bind_group)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FaceLightingUniform {
    positive: [f32; 3],
    _padding: u32,
    negative: [f32; 3],
    _padding2: u32,
}

impl From<&FaceLighting> for FaceLightingUniform {
    fn from(f: &FaceLighting) -> Self {
        FaceLightingUniform { 
            positive: [f.positive.x, f.positive.y, f.positive.z], 
            negative: [f.negative.x, f.negative.y, f.negative.z],
            _padding: 0,
            _padding2: 0,
        }
    }
}