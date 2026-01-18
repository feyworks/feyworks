use std::rc::Rc;

use kero::gfx::SubTexture;

/// A single renderable [`SpriteFont`](crate::SpriteFont) glyph.
#[derive(Debug, Clone)]
pub struct SpriteGlyph(Rc<Inner>);

#[derive(Debug)]
struct Inner {
    sub: Option<SubTexture>,
    advance: f32,
}

impl SpriteGlyph {
    #[inline]
    pub fn new(sub: Option<SubTexture>, advance: f32) -> Self {
        Self(Rc::new(Inner { sub, advance }))
    }

    #[inline]
    pub fn sub(&self) -> Option<&SubTexture> {
        self.0.sub.as_ref()
    }

    #[inline]
    pub fn advance(&self) -> f32 {
        self.0.advance
    }
}
