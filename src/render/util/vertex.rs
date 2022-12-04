use nalgebra::Point3;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexRaw {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
    pub tex_index: u32,
}

unsafe impl bytemuck::Pod for VertexRaw {}
unsafe impl bytemuck::Zeroable for VertexRaw {}

impl VertexRaw {
    pub const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Uint32];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub type Vertex = Point3<f32>;

pub trait VertexToRaw {
    fn get_raw(&self, tex_coord: &[f32; 2], tex_index: u32) -> VertexRaw;
}

impl VertexToRaw for Vertex {
    fn get_raw(&self, tex_coord: &[f32; 2], tex_index: u32) -> VertexRaw {
        VertexRaw { 
            position: [
                self.x,
                self.y,
                self.z,
            ],
            tex_coord: *tex_coord,
            tex_index,
        }
    }
}

// pub struct VertexIndexRaw {
//     pub index: u32,
//     pub tex_coords: [f32; 2],
//     pub tex_index: u32,
// }

// impl VertexIndexRaw {
//     pub fn new(index: u32, tex_coords: [f32; 2], tex_index: u32) -> Self {
//         Self {
//             index,
//             tex_coords,
//             tex_index,
//         }
//     }
// }
