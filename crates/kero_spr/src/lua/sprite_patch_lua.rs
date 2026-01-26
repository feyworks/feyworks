use crate::SpritePatch;
use fey_lua::{LuaModule, UserDataOf};
use kero::prelude::*;
use mlua::prelude::LuaResult;
use mlua::{Lua, UserData, UserDataMethods, UserDataRef, UserDataRefMut, Value};

pub struct SpritePatchModule;

pub type SpritePatchObj = UserDataOf<SpritePatch>;
pub type SpritePatchRef = UserDataRef<SpritePatch>;
pub type SpritePatchMut = UserDataRefMut<SpritePatch>;

impl LuaModule for SpritePatchModule {
    const PATH: &'static str = "SpritePatch";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for SpritePatchModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_, (tex, outer, inner): (TextureRef, RectF, RectF)| {
                Ok(SpritePatch::new(tex.clone(), outer, inner))
            },
        );
        add_methods(methods);
    }
}

impl UserData for SpritePatch {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function(
        "draw",
        |lua, (this, rect, col, mode): (SpritePatchRef, RectF, Option<Rgba8>, Option<ColorMode>)| {
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            this.draw_ext(draw, rect, col, mode);
            Ok(())
        },
    );
}
