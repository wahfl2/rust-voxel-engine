use super::block::Block;

pub struct Chunk {
    pub pos: ChunkPos,
    pub blocks: [[[Block; 16]; 16]; 16],
}

pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}