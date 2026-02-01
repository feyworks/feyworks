use crate::{Channel, FromRgb, FromRgba, Grey, GreyAlpha, Rgb, ToRgba};

use bytemuck::{Pod, Zeroable};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

/// An alias for [`Rgba<u8>`].
pub type Rgba8 = Rgba<u8>;

/// An alias for [`Rgba<u16>`].
pub type Rgba16 = Rgba<u16>;

/// An alias for [`Rgba<f32>`].
pub type Rgba32F = Rgba<f32>;

/// An alias for [`Rgba<f64>`].
pub type Rgba64F = Rgba<f64>;

/// A 4-channel RGBA color.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

fey_math::impl_approx!(
    NAME = Rgba
    FIELDS = (r, g, b, a)
);
fey_math::impl_casts!(
    NAME = Rgba
    FIELDS = (r, g, b, a)
);
fey_math::impl_tuple_arr!(
    NAME = Rgba
    LEN = 4
    FIELDS = (r, g, b, a)
    TUPLE = (T, T, T, T)
);

unsafe impl<T: Zeroable> Zeroable for Rgba<T> {}
unsafe impl<T: Pod> Pod for Rgba<T> {}

/// Create a new RGBA color.
#[inline]
pub const fn rgba<T>(r: T, g: T, b: T, a: T) -> Rgba<T> {
    Rgba { r, g, b, a }
}

impl<T> Rgba<T> {
    /// Create a new RGBA color.
    #[inline]
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: Channel> Rgba<T> {
    pub const TRANSPARENT: Self = Self::splat(T::ZERO);
    pub const BLACK: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::CHANNEL_MAX);
    pub const WHITE: Self = Self::splat(T::CHANNEL_MAX);
    pub const RED: Self = Self::new(T::CHANNEL_MAX, T::ZERO, T::ZERO, T::CHANNEL_MAX);
    pub const GREEN: Self = Self::new(T::ZERO, T::CHANNEL_MAX, T::ZERO, T::CHANNEL_MAX);
    pub const BLUE: Self = Self::new(T::ZERO, T::ZERO, T::CHANNEL_MAX, T::CHANNEL_MAX);
    pub const FUCHSIA: Self = Self::new(T::CHANNEL_MAX, T::ZERO, T::CHANNEL_MAX, T::CHANNEL_MAX);
    pub const CYAN: Self = Self::new(T::ZERO, T::CHANNEL_MAX, T::CHANNEL_MAX, T::CHANNEL_MAX);
    pub const YELLOW: Self = Self::new(T::CHANNEL_MAX, T::CHANNEL_MAX, T::ZERO, T::CHANNEL_MAX);

    /// Create a color with all components set to the same value.
    #[inline]
    pub const fn splat(val: T) -> Self {
        Self::new(val, val, val, val)
    }

    /// Convert from grey to RGBA.
    #[inline]
    pub fn from_grey<C: Channel>(val: Grey<C>) -> Self {
        val.to_rgba()
    }

    /// Convert from grey-alpha to RGBA.
    #[inline]
    pub fn from_grey_alpha<C: Channel>(val: GreyAlpha<C>) -> Self {
        val.to_rgba()
    }

    /// Unsigned-normalize multiply all channels by the value.
    #[inline]
    pub fn un_mul(self, a: T) -> Self {
        Self::new(
            self.r.un_mul(a),
            self.g.un_mul(a),
            self.b.un_mul(a),
            self.a.un_mul(a),
        )
    }

    /// Unsigned-normalize multiply all channels by the value.
    #[inline]
    pub fn mul_color(self, Self { r, g, b, a }: Self) -> Self {
        Self::new(
            self.r.un_mul(r),
            self.g.un_mul(g),
            self.b.un_mul(b),
            self.a.un_mul(a),
        )
    }

    /// Unsigned-normalize add all channels by the value.
    #[inline]
    pub fn add_color(self, Self { r, g, b, a }: Self) -> Self {
        Self::new(
            self.r.un_add(r),
            self.g.un_add(g),
            self.b.un_add(b),
            self.a.un_add(a),
        )
    }

    /// Unsigned-normalize subtract all channels by the value.
    #[inline]
    pub fn sub_color(self, Self { r, g, b, a }: Self) -> Self {
        Self::new(
            self.r.un_sub(r),
            self.g.un_sub(g),
            self.b.un_sub(b),
            self.a.un_sub(a),
        )
    }
}

#[inline]
const fn bgr(packed: u32) -> Rgba8 {
    Rgba8::new(packed as u8, (packed >> 8) as u8, (packed >> 16) as u8, 255)
}

impl Rgba<u8> {
    pub const ALICE_BLUE: Self = bgr(0xfff8f0);
    pub const ANTIQUE_WHITE: Self = bgr(0xd7ebfa);
    pub const AQUA: Self = bgr(0xffff00);
    pub const AQUAMARINE: Self = bgr(0xd4ff7f);
    pub const AZURE: Self = bgr(0xfffff0);
    pub const BEIGE: Self = bgr(0xdcf5f5);
    pub const BISQUE: Self = bgr(0xc4e4ff);
    pub const BLANCHED_ALMOND: Self = bgr(0xcdebff);
    pub const BLUE_VIOLET: Self = bgr(0xe22b8a);
    pub const BROWN: Self = bgr(0x2a2aa5);
    pub const BURLY_WOOD: Self = bgr(0x87b8de);
    pub const CADET_BLUE: Self = bgr(0xa09e5f);
    pub const CHARTREUSE: Self = bgr(0x00ff7f);
    pub const CHOCOLATE: Self = bgr(0x1e69d2);
    pub const CORAL: Self = bgr(0x507fff);
    pub const CORNFLOWER_BLUE: Self = bgr(0xed9564);
    pub const CORNSILK: Self = bgr(0xdcf8ff);
    pub const CRIMSON: Self = bgr(0x3c14dc);
    pub const DARK_BLUE: Self = bgr(0x8b0000);
    pub const DARK_CYAN: Self = bgr(0x8b8b00);
    pub const DARK_GOLDENROD: Self = bgr(0x0b86b8);
    pub const DARK_GRAY: Self = bgr(0xa9a9a9);
    pub const DARK_GREEN: Self = bgr(0x006400);
    pub const DARK_KHAKI: Self = bgr(0x6bb7bd);
    pub const DARK_MAGENTA: Self = bgr(0x8b008b);
    pub const DARK_OLIVE_GREEN: Self = bgr(0x2f6b55);
    pub const DARK_ORANGE: Self = bgr(0x008cff);
    pub const DARK_ORCHID: Self = bgr(0xcc3299);
    pub const DARK_RED: Self = bgr(0x00008b);
    pub const DARK_SALMON: Self = bgr(0x7a96e9);
    pub const DARK_SEA_GREEN: Self = bgr(0x8bbc8f);
    pub const DARK_SLATE_BLUE: Self = bgr(0x8b3d48);
    pub const DARK_SLATE_GRAY: Self = bgr(0x4f4f2f);
    pub const DARK_TURQUOISE: Self = bgr(0xd1ce00);
    pub const DARK_VIOLET: Self = bgr(0xd30094);
    pub const DEEP_PINK: Self = bgr(0x9314ff);
    pub const DEEP_SKY_BLUE: Self = bgr(0xffbf00);
    pub const DIM_GRAY: Self = bgr(0x696969);
    pub const DODGER_BLUE: Self = bgr(0xff901e);
    pub const FIREBRICK: Self = bgr(0x2222b2);
    pub const FLORAL_WHITE: Self = bgr(0xf0faff);
    pub const FOREST_GREEN: Self = bgr(0x228b22);
    pub const GAINSBORO: Self = bgr(0xdcdcdc);
    pub const GHOST_WHITE: Self = bgr(0xfff8f8);
    pub const GOLD: Self = bgr(0x00d7ff);
    pub const GOLDENROD: Self = bgr(0x20a5da);
    pub const GRAY: Self = bgr(0x808080);
    pub const GREEN_YELLOW: Self = bgr(0x2fffad);
    pub const HONEYDEW: Self = bgr(0xf0fff0);
    pub const HOT_PINK: Self = bgr(0xb469ff);
    pub const INDIAN_RED: Self = bgr(0x5c5ccd);
    pub const INDIGO: Self = bgr(0x82004b);
    pub const IVORY: Self = bgr(0xf0ffff);
    pub const KHAKI: Self = bgr(0x8ce6f0);
    pub const LAVENDER: Self = bgr(0xfae6e6);
    pub const LAVENDER_BLUSH: Self = bgr(0xf5f0ff);
    pub const LAWN_GREEN: Self = bgr(0x00fc7c);
    pub const LEMON_CHIFFON: Self = bgr(0xcdfaff);
    pub const LIGHT_BLUE: Self = bgr(0xe6d8ad);
    pub const LIGHT_CORAL: Self = bgr(0x8080f0);
    pub const LIGHT_CYAN: Self = bgr(0xffffe0);
    pub const LIGHT_GOLDENROD_YELLOW: Self = bgr(0xd2fafa);
    pub const LIGHT_GRAY: Self = bgr(0xd3d3d3);
    pub const LIGHT_GREEN: Self = bgr(0x90ee90);
    pub const LIGHT_PINK: Self = bgr(0xc1b6ff);
    pub const LIGHT_SALMON: Self = bgr(0x7aa0ff);
    pub const LIGHT_SEA_GREEN: Self = bgr(0xaab220);
    pub const LIGHT_SKY_BLUE: Self = bgr(0xface87);
    pub const LIGHT_SLATE_GRAY: Self = bgr(0x998877);
    pub const LIGHT_STEEL_BLUE: Self = bgr(0xdec4b0);
    pub const LIGHT_YELLOW: Self = bgr(0xe0ffff);
    pub const LIME_GREEN: Self = bgr(0x32cd32);
    pub const LINEN: Self = bgr(0xe6f0fa);
    pub const MAROON: Self = bgr(0x000080);
    pub const MEDIUM_AQUAMARINE: Self = bgr(0xaacd66);
    pub const MEDIUM_BLUE: Self = bgr(0xcd0000);
    pub const MEDIUM_ORCHID: Self = bgr(0xd355ba);
    pub const MEDIUM_PURPLE: Self = bgr(0xdb7093);
    pub const MEDIUM_SEA_GREEN: Self = bgr(0x71b33c);
    pub const MEDIUM_SLATE_BLUE: Self = bgr(0xee687b);
    pub const MEDIUM_SPRING_GREEN: Self = bgr(0x9afa00);
    pub const MEDIUM_TURQUOISE: Self = bgr(0xccd148);
    pub const MEDIUM_VIOLET_RED: Self = bgr(0x8515c7);
    pub const MIDNIGHT_BLUE: Self = bgr(0x701919);
    pub const MINT_CREAM: Self = bgr(0xfafff5);
    pub const MISTY_ROSE: Self = bgr(0xe1e4ff);
    pub const MOCCASIN: Self = bgr(0xb5e4ff);
    pub const MONO_GAME_ORANGE: Self = bgr(0x003ce7);
    pub const NAVAJO_WHITE: Self = bgr(0xaddeff);
    pub const NAVY: Self = bgr(0x800000);
    pub const OLD_LACE: Self = bgr(0xe6f5fd);
    pub const OLIVE: Self = bgr(0x008080);
    pub const OLIVE_DRAB: Self = bgr(0x238e6b);
    pub const ORANGE: Self = bgr(0x00a5ff);
    pub const ORANGE_RED: Self = bgr(0x0045ff);
    pub const ORCHID: Self = bgr(0xd670da);
    pub const PALE_GOLDENROD: Self = bgr(0xaae8ee);
    pub const PALE_GREEN: Self = bgr(0x98fb98);
    pub const PALE_TURQUOISE: Self = bgr(0xeeeeaf);
    pub const PALE_VIOLET_RED: Self = bgr(0x9370db);
    pub const PAPAYA_WHIP: Self = bgr(0xd5efff);
    pub const PEACH_PUFF: Self = bgr(0xb9daff);
    pub const PERU: Self = bgr(0x3f85cd);
    pub const PINK: Self = bgr(0xcbc0ff);
    pub const PLUM: Self = bgr(0xdda0dd);
    pub const POWDER_BLUE: Self = bgr(0xe6e0b0);
    pub const PURPLE: Self = bgr(0x800080);
    pub const ROSY_BROWN: Self = bgr(0x8f8fbc);
    pub const ROYAL_BLUE: Self = bgr(0xe16941);
    pub const SADDLE_BROWN: Self = bgr(0x13458b);
    pub const SALMON: Self = bgr(0x7280fa);
    pub const SANDY_BROWN: Self = bgr(0x60a4f4);
    pub const SEA_GREEN: Self = bgr(0x578b2e);
    pub const SEA_SHELL: Self = bgr(0xeef5ff);
    pub const SIENNA: Self = bgr(0x2d52a0);
    pub const SILVER: Self = bgr(0xc0c0c0);
    pub const SKY_BLUE: Self = bgr(0xebce87);
    pub const SLATE_BLUE: Self = bgr(0xcd5a6a);
    pub const SLATE_GRAY: Self = bgr(0x908070);
    pub const SNOW: Self = bgr(0xfafaff);
    pub const SPRING_GREEN: Self = bgr(0x7fff00);
    pub const STEEL_BLUE: Self = bgr(0xb48246);
    pub const TAN: Self = bgr(0x8cb4d2);
    pub const TEAL: Self = bgr(0x808000);
    pub const THISTLE: Self = bgr(0xd8bfd8);
    pub const TOMATO: Self = bgr(0x4763ff);
    pub const TURQUOISE: Self = bgr(0xd0e040);
    pub const VIOLET: Self = bgr(0xee82ee);
    pub const WHEAT: Self = bgr(0xb3def5);
    pub const WHITE_SMOKE: Self = bgr(0xf5f5f5);
    pub const YELLOW_GREEN: Self = bgr(0x32cd9a);

    /// Pack the color into a `u32` value.
    #[inline]
    pub const fn pack(self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | (self.a as u32)
    }

    /// Unpack the color from a `u32` value.
    #[inline]
    pub const fn unpack(packed: u32) -> Self {
        Self::new(
            (packed >> 24) as u8,
            (packed >> 16) as u8,
            (packed >> 8) as u8,
            packed as u8,
        )
    }
}

impl Debug for Rgba<u8> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#10x}", self.pack())
    }
}

impl Display for Rgba<u8> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

macro_rules! impl_debug {
    ($name:ty) => {
        impl Debug for Rgba<$name> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Rgba")
                    .field("r", &self.r)
                    .field("g", &self.g)
                    .field("b", &self.b)
                    .field("a", &self.a)
                    .finish()
            }
        }
    };
}

impl_debug!(u16);
impl_debug!(f32);
impl_debug!(f64);

impl<T: Channel> From<Grey<T>> for Rgba<T> {
    #[inline]
    fn from(value: Grey<T>) -> Self {
        Self::new(value.0, value.0, value.0, T::CHANNEL_MAX)
    }
}

impl<T: Channel> From<GreyAlpha<T>> for Rgba<T> {
    #[inline]
    fn from(GreyAlpha { g, a }: GreyAlpha<T>) -> Self {
        Self::new(g, g, g, a)
    }
}

impl<T: Channel> From<Rgb<T>> for Rgba<T> {
    #[inline]
    fn from(Rgb { r, g, b }: Rgb<T>) -> Self {
        Self::new(r, g, b, T::CHANNEL_MAX)
    }
}

impl<T: Channel, F: Channel> ToRgba<T> for Rgba<F> {
    #[inline]
    fn to_rgba(self) -> Rgba<T> {
        Rgba::new(
            self.r.to_channel(),
            self.g.to_channel(),
            self.b.to_channel(),
            self.a.to_channel(),
        )
    }
}

impl<T: Channel, F: Channel> FromRgba<F> for Rgba<T> {
    #[inline]
    fn from_rgba(val: Rgba<F>) -> Self {
        val.to_rgba()
    }
}

impl<T: Channel, F: Channel> FromRgb<F> for Rgba<T> {
    #[inline]
    fn from_rgb(val: Rgb<F>) -> Self {
        val.to_rgba()
    }
}

impl From<u32> for Rgba<u8> {
    #[inline]
    fn from(value: u32) -> Self {
        Self::unpack(value)
    }
}

impl Serialize for Rgba<u8> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("#{:08X}", self.pack()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Rgba<u8> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        if str.chars().next() != Some('#') {
            return Err(D::Error::custom(format!(
                "invalid color string {str:?}, must be a hex code starting with '#'"
            )));
        }
        u32::from_str_radix(&str[1..], 16)
            .map(Self::unpack)
            .map_err(D::Error::custom)
    }
}
