use mlua::{Result as LuaResult, String as LuaString, Table, UserData};

use super::{
    backend::Backend,
    constants::{BACKEND, PROMPT_CONTENT},
};
use crate::event::Event;
use crate::model;

#[derive(Debug, Clone)]
pub struct PromptMask {}

impl UserData for PromptMask {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "set",
            |ctx, (data, mask): (LuaString, Table)| -> LuaResult<bool> {
                let prompt_data: String = ctx.named_registry_value(PROMPT_CONTENT).unwrap();
                let mask_data = data.to_str().unwrap();
                if prompt_data != mask_data {
                    return Ok(false);
                }
                let prompt_mask = model::PromptMask::from(mask);
                let backend: Backend = ctx.named_registry_value(BACKEND)?;
                backend
                    .writer
                    .send(Event::SetPromptMask(prompt_mask))
                    .unwrap();
                Ok(true)
            },
        );
    }
}
