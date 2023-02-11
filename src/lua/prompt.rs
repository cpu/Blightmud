use mlua::{Function, Table, UserData};

use crate::event::Event;

use super::{
    backend::Backend,
    constants::{BACKEND, PROMPT_CONTENT, PROMPT_CURSOR_POS, PROMPT_INPUT_LISTENER_TABLE},
};

#[derive(Debug, Clone)]
pub struct Prompt {}

impl UserData for Prompt {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("set", |ctx, line: String| {
            let backend: Backend = ctx.named_registry_value(BACKEND)?;
            backend
                .writer
                .send(Event::SetPromptInput(line.clone()))
                .unwrap();
            ctx.set_named_registry_value(PROMPT_CONTENT, line)?;
            Ok(())
        });
        methods.add_function("set_cursor", |ctx, offset: u32| {
            let backend: Backend = ctx.named_registry_value(BACKEND)?;
            ctx.set_named_registry_value(PROMPT_CURSOR_POS, offset as usize)?;
            let offset = offset - 1; // NB: Converting from Lua 1-indexing.
            backend
                .writer
                .send(Event::SetPromptInputCursor(offset))
                .unwrap();
            Ok(())
        });
        methods.add_function("get", |ctx, ()| -> mlua::Result<String> {
            ctx.named_registry_value(PROMPT_CONTENT)
        });
        methods.add_function("get_cursor", |ctx, ()| -> mlua::Result<u32> {
            ctx.named_registry_value::<_, u32>(PROMPT_CURSOR_POS)
        });
        methods.add_function(
            "add_prompt_listener",
            |ctx, func: Function| -> mlua::Result<()> {
                let table: Table = ctx.named_registry_value(PROMPT_INPUT_LISTENER_TABLE)?;
                table.set(table.raw_len() + 1, func)?;
                Ok(())
            },
        );
    }
}
