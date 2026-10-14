use gilrs::Axis;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumString, FromRepr, IntoStaticStr, VariantArray, VariantNames};

/// A gamepad axis.
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    EnumCount,
    FromRepr,
    VariantArray,
    VariantNames,
    EnumString,
    IntoStaticStr,
)]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    DPadX,
    DPadY,
}

impl TryFrom<Axis> for GamepadAxis {
    type Error = ();

    #[inline]
    fn try_from(value: Axis) -> Result<Self, Self::Error> {
        Ok(match value {
            Axis::LeftStickX => Self::LeftX,
            Axis::LeftStickY => Self::LeftY,
            Axis::RightStickX => Self::RightX,
            Axis::RightStickY => Self::RightY,
            Axis::DPadX => Self::DPadX,
            Axis::DPadY => Self::DPadY,
            _ => return Err(()),
        })
    }
}
