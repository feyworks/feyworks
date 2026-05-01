use crate::{Direction, Numeric, RadiansF, Vec2, Vec2I, impl_temp};
use fey_lua::{LuaModule, Temp};
use mlua::prelude::LuaResult;
use mlua::{Either, FromLua, IntoLua, Lua, Value, Variadic};

impl_temp!(Vec2I Vec2IRef Vec2IMut);

pub struct Vec2IModule;

impl LuaModule for Vec2IModule {
    const PATH: &'static str = "Vec2I";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let module = Temp::<Vec2I>::register(lua, "Vec2I", |members| {
            // fields
            members.getter("x", |obj| obj.x)?;
            members.setter("x", |obj, val: i32| obj.x = val)?;
            members.getter("y", |obj| obj.y)?;
            members.setter("y", |obj, val: i32| obj.y = val)?;

            // operators
            members.op_tostring(|obj| format!("{obj}"))?;
            members.op_eq(|a, b: Vec2I| a == &b)?;
            members.op_add(|a, b: Vec2I| a + b)?;
            members.op_sub(|a, b: Vec2I| a - b)?;
            members.op_mul(|a, b: Either<Vec2I, i32>| match b {
                Either::Left(b) => a * b,
                Either::Right(b) => a * b,
            })?;
            members.op_div(|a, b: Either<Vec2I, i32>| match b {
                Either::Left(b) => a / b,
                Either::Right(b) => a / b,
            })?;
            members.op_unm(|obj| -*obj)?;

            // methods
            members.method_mut("set", |obj, (x, y): (i32, i32)| {
                obj.x = x;
                obj.y = y;
            })?;
            members.method_mut("set_to", |obj, p: Vec2I| {
                *obj = p;
            })?;
            members.method("abs", |obj, _: ()| obj.abs())?;
            members.method("angle", |obj, _: ()| RadiansF::from_vec2(obj.to_f32()))?;
            members.method("clamp", |obj, (min, max): (Vec2I, Vec2I)| {
                (*obj).clamp(min, max)
            })?;
            members.method("cross", |a, b: Vec2I| a.cross(b))?;
            members.method("dist", |a, b: Vec2I| a.to_f32().dist(b.to_f32()))?;
            members.method("man_dist", |a, b: Vec2I| a.man_dist(b))?;
            members.method("dot", |a, b: Vec2I| a.dot(b))?;
            members.method("is_zero", |obj, _: ()| obj.is_zero())?;
            members.method("len", |obj, _: ()| obj.to_f32().len())?;
            members.method("man_len", |obj, _: ()| obj.x.abs() + obj.y.abs())?;
            members.method("max", |obj, args: Variadic<Vec2I>| {
                args.into_iter().fold(*obj, |max, arg| max.max(arg))
            })?;
            members.method("min", |obj, args: Variadic<Vec2I>| {
                args.into_iter().fold(*obj, |min, arg| min.min(arg))
            })?;
            members.method("norm", |obj, _: ()| obj.to_f32().norm_safe())?;
            members.method("sign", |obj, _: ()| obj.signum())?;
            members.method("sqr_dist", |a, b: Vec2I| a.sqr_dist(b))?;
            members.method("sqr_len", |obj, _: ()| obj.sqr_len())?;
            members.method("turn_left", |obj, _: ()| obj.turn_left())?;
            members.method("turn_right", |obj, _: ()| obj.turn_right())?;
            members.method("with_len", |obj, new_len: f32| {
                obj.to_f32().norm_safe() * new_len
            })?;
            members.method("with_x", |obj, val: i32| obj.with_x(val))?;
            members.method("with_y", |obj, val: i32| obj.with_y(val))?;
            members.method("with_z", |obj, val: i32| obj.with_z(val))?;
            members.method("yx", |obj, _: ()| obj.yx())?;
            Ok(())
        })?;

        module.set_metatable(Some({
            let meta = lua.create_table()?;
            meta.set(
                "__call",
                lua.create_function(|_, (_, x, y): (Value, i32, i32)| Ok(Vec2I::new(x, y)))?,
            )?;
            meta
        }))?;

        macro_rules! constant {
            ($name:literal $var:ident) => {
                module.set($name, lua.create_function(|_, _: ()| Ok(Vec2I::$var))?)?;
            };
        }
        constant!("zero" ZERO);
        constant!("one" ONE);
        constant!("x_axis" X_AXIS);
        constant!("y_axis" Y_AXIS);
        constant!("right" RIGHT);
        constant!("left" LEFT);
        constant!("down" DOWN);
        constant!("up" UP);
        constant!("east" EAST);
        constant!("south" SOUTH);
        constant!("west" WEST);
        constant!("north" NORTH);

        module.set(
            "new",
            lua.create_function(|_, (x, y): (i32, i32)| Ok(Vec2I::new(x, y)))?,
        )?;
        module.set(
            "splat",
            lua.create_function(|_, val: i32| Ok(Vec2I::splat(val)))?,
        )?;

        // module.set(
        //     "persistent",
        //     lua.create_function(|lua, (x, y): (f32, f32)| {
        //         lua.create_any_userdata(Self::new(x, y))
        //     })?,
        // )?;

        Ok(Value::Table(module))
    }
}

macro_rules! impl_from_to {
    ($to:ident $to_fn:ident) => {
        impl FromLua for Vec2<$to> {
            #[inline]
            fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
                Vec2I::from_lua(value, lua).map(Vec2::$to_fn)
            }
        }

        impl IntoLua for Vec2<$to> {
            #[inline]
            fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
                self.to_i32().into_lua(lua)
            }
        }
    };
}

impl_from_to!(u8 to_u8);
impl_from_to!(i8 to_i8);
impl_from_to!(u16 to_u16);
impl_from_to!(i16 to_i16);
impl_from_to!(u32 to_u32);
impl_from_to!(u64 to_u64);
impl_from_to!(i64 to_i64);
impl_from_to!(usize to_usize);
impl_from_to!(isize to_isize);
