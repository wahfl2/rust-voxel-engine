use std::{fs::File, io::Read};

use image::DynamicImage;

use super::vertex::Vertex;

pub struct CubeModel {
    pub textures: Vec<DynamicImage>,
    pub face_textures: [usize; 6],
}

impl CubeModel {
    pub const VERTICES: &[Vertex] = &[
        Vertex::new(0.0, 0.0, 0.0),
        Vertex::new(1.0, 0.0, 0.0),
        Vertex::new(1.0, 0.0, 1.0),
        Vertex::new(0.0, 0.0, 1.0),
        Vertex::new(0.0, 1.0, 0.0),
        Vertex::new(1.0, 1.0, 0.0),
        Vertex::new(1.0, 1.0, 1.0),
        Vertex::new(0.0, 1.0, 1.0),
    ];

    pub const INDICES: &[u16] = &[
        // Bottom 0, 1, 2, 3, 
        2, 1, 0,
        3, 2, 0,
        // Top 4, 5, 6, 7,
        4, 5, 6,
        4, 6, 7,
        // Side faces 0, 1, 5, 4, 
        0, 1, 5,
        0, 5, 4,
        // 1, 2, 6, 5,
        1, 2, 6,
        1, 6, 5,
        // 2, 3, 7, 6,
        2, 3, 7,
        2, 7, 6,
        // 3, 0, 4, 7,
        3, 0, 4,
        3, 4, 7,
    ];

    pub const DEFAULT_UV_MAP: &[[f32; 2]] = &[
        [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
        [0.0, 1.0], [1.0, 1.0], [0.0, 0.0],

        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
        [0.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
        [0.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
        [0.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
        [0.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
        [0.0, 0.0], [1.0, 1.0], [0.0, 1.0],
    ];

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