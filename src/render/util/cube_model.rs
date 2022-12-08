use std::{fs::File, io::Read};

use image::DynamicImage;
use nalgebra::Vector3;
use once_cell::sync::Lazy;

use super::shapes::Quad;

/// Quads of a cube model with width 1.0 centered at the origin
pub static DEFAULT_CUBE_MODEL_QUADS: Lazy<[Quad; 6]> = Lazy::new(|| {[
    Quad::new_x_center(Vector3::new(0.5, 0.0, 0.0), 1.0, 1.0, true),
    Quad::new_x_center(Vector3::new(-0.5, 0.0, 0.0), 1.0, 1.0, false),
    Quad::new_y_center(Vector3::new(0.0, 0.5, 0.0), 1.0, 1.0, true),
    Quad::new_y_center(Vector3::new(0.0, -0.5, 0.0), 1.0, 1.0, false),
    Quad::new_z_center(Vector3::new(0.0, 0.0, 0.5), 1.0, 1.0, true),
    Quad::new_z_center(Vector3::new(0.0, 0.0, -0.5), 1.0, 1.0, false),
]});

pub struct CubeModel {
    pub textures: Vec<DynamicImage>,
    pub face_textures: [usize; 6],
}

impl CubeModel {
    pub fn new(texture_path: &'static str) -> Self {
        let path = format!("assets/{}", texture_path);
        println!("{}", path);
        let file = File::open(path).unwrap();
        let image_tex = image::load_from_memory(
            file.bytes().map(|b| { b.unwrap() })
            .collect::<Vec<_>>().as_slice()
        ).unwrap();

        Self {
            textures: vec![image_tex],
            face_textures: [0; 6],
        }
    }
}

impl Default for CubeModel {
    fn default() -> Self {
        Self::new("stone.png")
    }
}