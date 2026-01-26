use crate::Sprite;
use fey_lua::{LuaModule, UserDataOf};
use kero::prelude::*;
use mlua::prelude::LuaResult;
use mlua::{Lua, UserData, UserDataMethods, UserDataRef, UserDataRefMut, Value};

pub struct SpriteModule;

pub type SpriteObj = UserDataOf<Sprite>;
pub type SpriteRef = UserDataRef<Sprite>;
pub type SpriteMut = UserDataRefMut<Sprite>;

impl LuaModule for SpriteModule {
    const PATH: &'static str = "Sprite";

    fn load(lua: &Lua) -> LuaResult<Value> {
        lua.create_userdata(Self).map(Value::UserData)
    }
}

impl UserData for SpriteModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_, (tex, rect, off, size): (TextureRef, RectF, Option<Vec2F>, Option<Vec2F>)| {
                let off = off.unwrap_or(Vec2F::ZERO);
                let size = size.unwrap_or_else(|| rect.size());
                Ok(Sprite::new_ext(tex.clone(), rect, off, size))
            },
        );
        add_methods(methods);
    }
}

impl UserData for Sprite {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_methods(methods);
    }
}

fn add_methods<T, M: UserDataMethods<T>>(methods: &mut M) {
    methods.add_function("rect", |_, this: SpriteRef| Ok(this.sub.rect));
    methods.add_function("offset", |_, this: SpriteRef| Ok(this.sub.offset));
    methods.add_function("size", |_, this: SpriteRef| Ok(this.sub.size));
    methods.add_function("coords", |_, this: SpriteRef| {
        let [a, b, c, d] = this.sub.coords;
        Ok((a, b, c, d))
    });
    methods.add_function(
        "draw",
        |lua,
         (this, pos, col, mode, fx, fy): (
            SpriteRef,
            Vec2F,
            Option<Rgba8>,
            Option<ColorMode>,
            Option<bool>,
            Option<bool>,
        )| {
            let col = col.unwrap_or(Rgba8::WHITE);
            let mode = mode.unwrap_or(ColorMode::MULT);
            let draw = Draw::from_lua(lua)?;
            match (fx, fy) {
                (None, None) => {
                    draw.subtexture_at_ext(&this.sub, pos, col, mode);
                }
                (fx, fy) => {
                    let fx = fx.unwrap_or(false);
                    let fy = fy.unwrap_or(false);
                    draw.subtexture_at_flipped(&this.sub, pos, col, mode, (fx, fy));
                }
            }
            Ok(())
            //
        },
    );
}
