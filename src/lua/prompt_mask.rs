use mlua::{Table, UserData};

use super::{backend::Backend, constants::BACKEND};
use crate::event::Event;
use crate::model;

#[derive(Debug, Clone)]
pub struct PromptMask {}

impl UserData for PromptMask {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("set", |ctx, mask: Table| {
            let prompt_mask = model::PromptMask::from(mask);
            let backend: Backend = ctx.named_registry_value(BACKEND)?;
            backend
                .writer
                .send(Event::SetPromptMask(prompt_mask))
                .unwrap();
            Ok(())
        });
    }
}
