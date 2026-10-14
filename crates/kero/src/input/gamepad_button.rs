use gilrs::Button;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumString, FromRepr, IntoStaticStr, VariantArray, VariantNames};

/// A gamepad button.
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
pub enum GamepadButton {
    South,
    East,
    West,
    North,
    Select,
    Menu,
    Start,
    LeftThumb,
    RightThumb,
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

impl TryFrom<Button> for GamepadButton {
    type Error = ();

    #[inline]
    fn try_from(value: Button) -> Result<Self, Self::Error> {
        Ok(match value {
            Button::South => Self::South,
            Button::East => Self::East,
            Button::North => Self::North,
            Button::West => Self::West,
            Button::LeftTrigger => Self::LeftBumper,
            Button::LeftTrigger2 => Self::LeftTrigger,
            Button::RightTrigger => Self::RightBumper,
            Button::RightTrigger2 => Self::RightTrigger,
            Button::Select => Self::Select,
            Button::Start => Self::Start,
            Button::Mode => Self::Menu,
            Button::LeftThumb => Self::LeftThumb,
            Button::RightThumb => Self::RightThumb,
            Button::DPadUp => Self::DPadUp,
            Button::DPadDown => Self::DPadDown,
            Button::DPadLeft => Self::DPadLeft,
            Button::DPadRight => Self::DPadRight,
            Button::C | Button::Z | Button::Unknown => return Err(()),
        })
    }
}
