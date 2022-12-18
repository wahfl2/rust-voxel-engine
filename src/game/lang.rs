use rustc_hash::FxHashMap;
use serde::{Deserialize, de::{Visitor, MapAccess}};

pub struct LangVisitor;
impl<'de> Visitor<'de> for LangVisitor {
    type Value = LangJson;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("lang.json")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = FxHashMap::default();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(LangJson { data: map })
    }
}

pub struct LangJson {
    pub data: FxHashMap<String, String>,
}

impl<'de> Deserialize<'de> for LangJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_map(LangVisitor)
    }
}