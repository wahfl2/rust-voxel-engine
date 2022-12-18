use std::{fs::File, io::Read};

use guillotiere::AtlasAllocator;
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
    pub fn new(texture_path: &str) -> Self {
        let path = format!("assets/{}", texture_path);
        println!("{}", path);
        let file = File::open(path).unwrap();
        let image_tex = image::load_from_memory(
            file.bytes().map(|b| { b.unwrap() })
            .collect::<Vec<_>>().as_slice()
        ).unwrap();
        
        let mut textures = Vec::new();
        let face_textures;

        let img_height = image_tex.height();
        let img_width = image_tex.width();
        let aspect_ratio = img_height as f32 / img_width as f32;
        
        if aspect_ratio == 3.0 {
            let single_height = img_width;

            // Top
            textures.push(image_tex.crop_imm(0, 0, img_width, single_height));
            // Sides
            textures.push(image_tex.crop_imm(0, single_height, img_width, single_height));
            // Bottom
            textures.push(image_tex.crop_imm(0, 2 * single_height, img_width, single_height));

            face_textures = [0, 1, 1, 1, 1, 2];
        } else {
            textures.push(image_tex);
            face_textures = [0; 6];
        }

        Self {
            textures,
            face_textures,
        }
    }
}

impl Default for CubeModel {
    fn default() -> Self {
        Self::new("stone.png")
    }
}