use std::ops::Deref;
use std::rc::Rc;

use kero::gfx::{SubTexture, Texture};
use kero::math::{RectF, Vec2F};

/// A single renderable sprite.
#[derive(Debug, Clone)]
pub struct Sprite(Rc<SubTexture>);

impl From<SubTexture> for Sprite {
    fn from(value: SubTexture) -> Self {
        Self(Rc::new(value))
    }
}

impl Sprite {
    #[inline]
    pub fn new_ext(texture: Texture, rect: RectF, offset: Vec2F, size: Vec2F) -> Self {
        Self::from(SubTexture::new_ext(texture, rect, offset, size))
    }

    #[inline]
    pub fn new(texture: Texture, rect: RectF) -> Self {
        Self::from(SubTexture::new(texture, rect))
    }
}

impl Deref for Sprite {
    type Target = SubTexture;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
