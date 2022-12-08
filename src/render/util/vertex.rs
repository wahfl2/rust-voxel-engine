use nalgebra::{Vector3, Vector2};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexRaw {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
    pub tex_index: u32,
}

unsafe impl bytemuck::Pod for VertexRaw {}
unsafe impl bytemuck::Zeroable for VertexRaw {}

impl VertexRaw {
    pub const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x3, 3 => Float32x3, 1 => Float32x2, 2 => Uint32];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
    pub texture_id: u32,
}

impl Vertex {
    pub fn new(pos: Vector3<f32>, normal: Vector3<f32>, tex_coord: Vector2<f32>, texture_id: u32) -> Self {
        Self {
            pos,
            normal,
            tex_coord,
            texture_id,
        }
    }

    pub fn get_raw(&self) -> VertexRaw {
        VertexRaw { 
            position: [
                self.pos.x,
                self.pos.y,
                self.pos.z,
            ],
            normal: [
                self.normal.x,
                self.normal.y,
                self.normal.z,
            ],
            tex_coord: [
                self.tex_coord.x,
                self.tex_coord.y,
            ],
            tex_index: self.texture_id,
        }
    }
}

impl From<Vertex> for VertexRaw {
    fn from(v: Vertex) -> Self {
        Self { 
            position: [
                v.pos.x,
                v.pos.y,
                v.pos.z,
            ],
            normal: [
                v.normal.x,
                v.normal.y,
                v.normal.z,
            ],
            tex_coord: [
                v.tex_coord.x,
                v.tex_coord.y,
            ],
            tex_index: v.texture_id,
        }
    }
}