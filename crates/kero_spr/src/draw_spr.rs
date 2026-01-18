use crate::SpriteFont;
use kero::prelude::*;

/// Extension for [`Draw`](kero::gfx::Draw) with sprite rendering methods.
pub trait DrawSpr {
    /// Draw a sprite at the provided position.
    fn sprite_at_flipped(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    );

    /// Draw a sprite at the provided position.
    fn sprite_at_ext(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
    );

    /// Draw a sprite at the provided position.
    #[inline]
    fn sprite_at(&mut self, sub: impl AsRef<SubTexture>, pos: impl Into<Vec2F>) {
        self.sprite_at_ext(sub, pos, Rgba8::WHITE, ColorMode::MULT);
    }

    /// Draw text using a sprite font.
    fn sprite_text(&mut self, text: &str, pos: Vec2F, font: &SpriteFont, color: Rgba8);
}

impl DrawSpr for Draw {
    #[inline]
    fn sprite_at_flipped(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
        flip: impl Into<Vec2<bool>>,
    ) {
        self.subtexture_at_flipped(sub, pos, color, mode, flip);
    }

    #[inline]
    fn sprite_at_ext(
        &mut self,
        sub: impl AsRef<SubTexture>,
        pos: impl Into<Vec2F>,
        color: Rgba8,
        mode: ColorMode,
    ) {
        self.subtexture_at_ext(sub, pos, color, mode);
    }

    fn sprite_text(&mut self, text: &str, mut pos: Vec2F, font: &SpriteFont, color: Rgba8) {
        let left = pos.x;
        for chr in text.chars() {
            if chr == '\n' {
                pos.x = left;
                pos.y += font.line_height();
            } else if let Some(g) = font.glyph(chr).or_else(|| font.glyph('\0')) {
                if let Some(sub) = g.sub() {
                    self.subtexture_at_ext(sub, pos, color, ColorMode::MULT);
                }
                pos.x += g.advance();
            } else {
                println!("no glyph for: [{}]", chr);
            }
        }
    }
}
