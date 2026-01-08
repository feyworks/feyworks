mod handle;
mod handle_ref;
mod lua_module;
mod ops;
mod temp;
mod temp_members;
mod temp_types;
mod user_data_of;

pub use handle::*;
pub use handle_ref::*;
pub use lua_module::*;
pub use temp::*;
pub use temp_members::*;
pub(crate) use temp_types::*;
pub use user_data_of::*;

pub trait LuaExt {
    fn create_fill(&self, fill: Option<mlua::Table>) -> mlua::prelude::LuaResult<mlua::Table>;
}

impl LuaExt for mlua::Lua {
    #[inline]
    fn create_fill(&self, fill: Option<mlua::Table>) -> mlua::prelude::LuaResult<mlua::Table> {
        match fill {
            Some(fill) => {
                fill.clear()?;
                Ok(fill)
            }
            None => self.create_table(),
        }
    }
}
