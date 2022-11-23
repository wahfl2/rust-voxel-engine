use nalgebra::{Point3, Vector3, Matrix4, Affine3, Isometry3, UnitQuaternion, Quaternion};

use super::util::math::perspective;

pub struct Camera {
    pub origin: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, -10.0), 
            target: Point3::new(0.0, 0.0, 0.0), 
            up: Vector3::new(0.0, 1.0, 0.0),
            aspect: 1.0,
            fov_y: 45.0, 
            z_near: 0.01, 
            z_far: 10000.0,
        }
    }
}

impl Camera {
    pub fn calculate_projection_matrix(&self) -> Matrix4<f32> {
        let matr = Isometry3::look_at_rh(&self.origin, &self.target, &self.up).to_matrix();
        let proj = perspective(self.fov_y, self.aspect, self.z_near, self.z_far);
        proj * matr
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