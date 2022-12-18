use rustc_hash::FxHashMap;

use crate::render::util::cube_model::CubeModel;

pub struct InitBlockData {
    pub name: &'static str,
    pub model: CubeModel, // TODO: Custom models besides just cubes
}

pub struct StaticBlockData {
    inner: FxHashMap<&'static str, InitBlockData>,
}

impl StaticBlockData {
    // pub fn load() -> Self {

    // }
}