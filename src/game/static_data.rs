use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::render::util::cube_model::CubeModel;

use super::lang::LangJson;

pub struct InitBlockData {
    pub name: String,
    pub model: CubeModel, // TODO: Custom models besides just cubes
}

pub struct StaticBlockData {
    inner: FxHashMap<String, InitBlockData>,
}

impl StaticBlockData {
    pub fn load() -> Self {
        let data = include_str!("../../assets/en_us.json");
        let lang: LangJson = serde_json::from_str(data).unwrap();
        let mut hash = FxHashMap::default();

        for (id, name) in lang.data {
            hash.insert(
                id.clone(), 
                InitBlockData {
                    name,
                    model: CubeModel::new(&id),
                }
            );
        }

        Self {
            inner: hash
        }
    }
}

#[test]
pub fn test() {
    StaticBlockData::load();
}