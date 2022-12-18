pub struct Block {
    pub id: &'static str,
    pub data: Option<BlockData>,
}

pub struct BlockData {
    
}

impl Block {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            data: None,
        }
    }
}