use nalgebra::Matrix4;

pub fn perspective(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Matrix4<f32> {
    let f = (fov_y / 2.0).tan().recip();

    Matrix4::new(
        f / aspect, 0.0, 0.0, 0.0, 
        0.0, f, 0.0, 0.0, 
        0.0, 0.0, (z_far + z_near) / (z_near - z_far), (2.0 * z_far * z_near) / (z_near - z_far), 
        0.0, 0.0, -1.0, 0.0
    )
}