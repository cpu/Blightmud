use anyhow::{anyhow, Result};
use hunspell_rs::{CheckResult, Hunspell};
use mlua::prelude::LuaError;
use mlua::{AnyUserData, Result as LuaResult, String as LuaString, Table, UserData};
use std::sync::Arc;

pub const LUA_GLOBAL_NAME: &str = "spellcheck";

pub struct Spellchecker {
    hunspell: Option<HunspellSafe>,
}

impl Spellchecker {
    pub fn new() -> Self {
        Spellchecker { hunspell: None }
    }

    pub fn init(&mut self, aff_path: &str, dict_path: &str) {
        self.hunspell
            .replace(HunspellSafe::from(Hunspell::new(aff_path, dict_path)));
    }

    fn check_initialized(&self) -> Result<()> {
        if self.hunspell.is_none() {
            return Err(anyhow!("spellchecker not initialized"));
        }
        Ok(())
    }

    pub fn check(&self, word: &str) -> Result<bool> {
        self.check_initialized()?;
        match self.hunspell.as_ref().unwrap().check(word) {
            CheckResult::MissingInDictionary => Ok(false),
            _ => Ok(true),
        }
    }

    pub fn suggest(&self, word: &str) -> Result<Vec<String>> {
        self.check_initialized()?;
        Ok(self.hunspell.as_ref().unwrap().suggest(word))
    }
}

impl UserData for Spellchecker {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "init",
            |ctx, (aff_path, dict_path): (LuaString, LuaString)| -> LuaResult<()> {
                let this_aux = ctx.globals().get::<_, AnyUserData>(LUA_GLOBAL_NAME)?;
                let mut this = this_aux
                    .borrow_mut::<Spellchecker>()
                    .map_err(LuaError::external)?;
                this.init(aff_path.to_str()?, dict_path.to_str()?);
                Ok(())
            },
        );
        methods.add_function("check", |ctx, word: LuaString| -> LuaResult<bool> {
            let this_aux = ctx.globals().get::<_, AnyUserData>(LUA_GLOBAL_NAME)?;
            let this = this_aux
                .borrow::<Spellchecker>()
                .map_err(LuaError::external)?;
            let found = this.check(word.to_str()?).map_err(LuaError::external)?;
            Ok(found)
        });
        methods.add_function("suggest", |ctx, word: LuaString| -> LuaResult<Table> {
            let this_aux = ctx.globals().get::<_, AnyUserData>(LUA_GLOBAL_NAME)?;
            let this = this_aux
                .borrow::<Spellchecker>()
                .map_err(LuaError::external)?;
            let res_table = ctx.create_table()?;
            this.suggest(word.to_str()?)
                .map_err(LuaError::external)?
                .iter()
                .enumerate()
                .for_each(|(i, v)| res_table.set(i, v.as_str()).unwrap());
            Ok(res_table)
        });
    }
}

#[derive(Clone)]
struct HunspellSafe(Arc<Hunspell>);

unsafe impl Send for HunspellSafe {}

impl std::ops::Deref for HunspellSafe {
    type Target = Hunspell;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Hunspell> for HunspellSafe {
    fn from(hunspell: Hunspell) -> Self {
        Self(Arc::new(hunspell))
    }
}
