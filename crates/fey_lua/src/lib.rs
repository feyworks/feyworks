//! Temp types and helpers for Lua integration.

mod create_fill;
mod handle;
mod handle_ref;
mod instant_lua;
mod lua_module;
mod ops;
mod temp;
mod temp_members;
mod temp_types;
mod user_data_of;

pub use create_fill::*;
pub use handle::*;
pub use handle_ref::*;
pub use instant_lua::*;
pub use lua_module::*;
pub use temp::*;
pub use temp_members::*;
pub use temp_types::*;
pub use user_data_of::*;

pub use mlua;

/// Implement common userdata aliases on a userdata type.
///
/// For example:
///
/// ```
/// struct TestData;
///
/// impl mlua::UserData for TestData {}
///
/// impl_refs!(TestData, TestObj, TestRef, TestMut);
///
/// // produces:
/// //
/// // pub type TestObj = UserDataOf<TestData>;
/// // pub type TestRef = mlua::UserDataRef<TestData>;
/// // pub type TestMut = mlua::UserDataRefMut<TestData>;
/// ```
#[macro_export]
macro_rules! impl_refs {
    ($name:ident, $obj_name:ident, $ref_name:ident, $mut_name:ident) => {
        pub type $obj_name = $crate::UserDataOf<$name>;
        pub type $ref_name = $crate::mlua::UserDataRef<$name>;
        pub type $mut_name = $crate::mlua::UserDataRefMut<$name>;
    };
}
