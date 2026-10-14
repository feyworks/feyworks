use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumString, FromRepr, IntoStaticStr, VariantArray, VariantNames};
use winit::event::MouseButton as Winit;

/// A mouse button.
#[derive(
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    EnumCount,
    FromRepr,
    VariantArray,
    VariantNames,
    EnumString,
    IntoStaticStr,
)]
pub enum MouseButton {
    Left = 0,
    Middle,
    Right,
}

impl TryFrom<Winit> for MouseButton {
    type Error = Winit;

    #[inline]
    fn try_from(value: Winit) -> Result<Self, Self::Error> {
        Ok(match value {
            Winit::Left => Self::Left,
            Winit::Middle => Self::Middle,
            Winit::Right => Self::Right,
            x => return Err(x),
        })
    }
}
