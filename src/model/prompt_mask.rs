use log::debug;
use mlua::{Integer as LuaInt, String as LuaString, Table as LuaTable};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PromptMask {
    mask: BTreeMap<i32, String>,
}

impl PromptMask {
    pub fn new() -> Self {
        PromptMask {
            mask: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.mask.clear()
    }

    pub fn mask_buffer(&self, buf: &[char]) -> String {
        debug!("masking buf: {:?}", buf);
        let mut masked_buf = buf.to_owned();
        let mut offset = 0;
        for (idx, mask) in self.mask.iter() {
            // NB: idx subtracted by one to account for Lua one-indexing.
            let adjusted_idx = offset + (idx - 1) as usize;
            debug!(
                "idx: {}, mask: {:?}, len: {}, adjusted_idx: {}",
                idx,
                mask,
                mask.len(),
                adjusted_idx
            );
            masked_buf.splice(adjusted_idx..adjusted_idx, mask.chars());
            offset += mask.len();
        }

        masked_buf.iter().collect()
    }
}

impl From<BTreeMap<i32, String>> for PromptMask {
    fn from(mask: BTreeMap<i32, String>) -> Self {
        PromptMask { mask }
    }
}

impl From<LuaTable<'_>> for PromptMask {
    fn from(mask_table: LuaTable) -> Self {
        let mut mask = BTreeMap::new();
        for pair in mask_table.pairs::<LuaInt, LuaString>() {
            let (offset, marker) = pair.unwrap();
            mask.insert(offset as i32, marker.to_str().unwrap().to_string());
        }
        PromptMask { mask }
    }
}
