use crate::{
    AnimFrame, AnimLayer, AnimTag, Sprite, SpriteAnim, SpriteFont, SpriteGlyph, SpritePatch,
    SpriteSheet,
};
use kero::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

// Represents a packed sprite atlas.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteAtlas<I> {
    pub sprites: Vec<AtlasSprite<I>>,
    pub sheets: Vec<AtlasSheet<I>>,
    pub fonts: Vec<AtlasFont<I>>,
    pub patches: Vec<AtlasPatch<I>>,
    pub anims: Vec<AtlasAnim<I>>,
}

/// A packed sprite.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasSprite<I> {
    pub id: I,
    pub size: Vec2U,
    pub rect: RectU,
    pub off: Vec2<i32>,
}

/// A packed sheet.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasSheet<I> {
    pub id: I,
    pub tile_size: Vec2U,
    pub size: Vec2U,
    pub tiles: Vec<Option<AtlasTile>>,
}

/// A packed sheet tile.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasTile {
    pub rect: RectU,
    pub off: Vec2<i32>,
}

/// A packed font.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasFont<I> {
    pub id: I,
    pub ascent: i32,
    pub descent: i32,
    pub line_gap: i32,
    pub glyphs: Vec<AtlasGlyph>,
    pub kerning: Vec<(char, char, i32)>,
}

/// A packed font glyph.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasGlyph {
    pub chr: char,
    pub adv: i32,
    pub size: Vec2U,
    pub rect: RectU,
    pub off: Vec2<i32>,
}

/// A packed 9-patch.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasPatch<I> {
    pub id: I,
    pub outer: RectU,
    pub inner: RectU,
}

/// A packed animation.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasAnim<I> {
    pub id: I,
    pub size: Vec2U,
    pub cels: Vec<AtlasCel>,
    pub frames: Vec<AnimFrame>,
    pub tags: Vec<AnimTag>,
    pub layers: Vec<AnimLayer>,
}

/// A packed animation cel.
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasCel {
    pub size: Vec2U,
    pub rect: RectU,
    pub off: Vec2<i32>,
}

/// Graphics assets generated from a sprite atlas.
pub struct AtlasGraphics<I> {
    pub texture: Texture,
    pub sprites: Vec<(I, Sprite)>,
    pub sheets: Vec<(I, SpriteSheet)>,
    pub fonts: Vec<(I, SpriteFont)>,
    pub patches: Vec<(I, SpritePatch)>,
    pub anims: Vec<(I, SpriteAnim)>,
}

impl<I: Eq + Hash> AtlasGraphics<I> {
    /// Convert the graphics lists into hashmaps.
    pub fn mapped(self) -> AtlasGraphicsMapped<I> {
        AtlasGraphicsMapped {
            texture: self.texture,
            sprites: self.sprites.into_iter().collect(),
            sheets: self.sheets.into_iter().collect(),
            fonts: self.fonts.into_iter().collect(),
            patches: self.patches.into_iter().collect(),
            anims: self.anims.into_iter().collect(),
        }
    }
}

/// Hash-mapped graphics assets generated from a sprite atlas.
pub struct AtlasGraphicsMapped<I> {
    pub texture: Texture,
    pub sprites: HashMap<I, Sprite>,
    pub sheets: HashMap<I, SpriteSheet>,
    pub fonts: HashMap<I, SpriteFont>,
    pub patches: HashMap<I, SpritePatch>,
    pub anims: HashMap<I, SpriteAnim>,
}

impl<I> SpriteAtlas<I> {
    /// Create renderable graphics assets from this sprite atlas.
    pub fn create_graphics(self, texture: Texture) -> AtlasGraphics<I> {
        let sprites = self
            .sprites
            .into_iter()
            .map(|sprite| {
                (
                    sprite.id,
                    Sprite::new_ext(
                        texture.clone(),
                        sprite.rect.to_f32(),
                        sprite.off.to_f32(),
                        sprite.size.to_f32(),
                    ),
                )
            })
            .collect();

        let sheets = self
            .sheets
            .into_iter()
            .map(|sheet| {
                let tile_size = sheet.tile_size.to_f32();
                (
                    sheet.id,
                    SpriteSheet {
                        tiles: VecGrid::with_store(
                            sheet.size,
                            sheet
                                .tiles
                                .into_iter()
                                .map(|tile| {
                                    tile.map(|tile| {
                                        Sprite::new_ext(
                                            texture.clone(),
                                            tile.rect.to_f32(),
                                            tile.off.to_f32(),
                                            tile_size,
                                        )
                                    })
                                })
                                .collect(),
                        ),
                        tile_size,
                    },
                )
            })
            .collect();

        let fonts = self
            .fonts
            .into_iter()
            .map(|font| {
                (
                    font.id,
                    SpriteFont {
                        ascent: font.ascent as f32,
                        descent: font.descent as f32,
                        line_gap: font.line_gap as f32,
                        glyphs: font
                            .glyphs
                            .into_iter()
                            .map(|g| {
                                (
                                    g.chr,
                                    SpriteGlyph {
                                        sprite: (g.size.x > 0).then(|| {
                                            Sprite::new_ext(
                                                texture.clone(),
                                                g.rect.to_f32(),
                                                g.off.to_f32(),
                                                g.size.to_f32(),
                                            )
                                        }),
                                        advance: g.adv as f32,
                                    },
                                )
                            })
                            .collect(),
                        kerning: font
                            .kerning
                            .into_iter()
                            .map(|(a, b, k)| ((a, b), k as f32))
                            .collect(),
                    },
                )
            })
            .collect();

        let patches = self
            .patches
            .into_iter()
            .map(|patch| {
                (
                    patch.id,
                    SpritePatch::new(texture.clone(), patch.outer.to_f32(), patch.inner.to_f32()),
                )
            })
            .collect();

        let anims = self
            .anims
            .into_iter()
            .map(|anim| {
                (anim.id, {
                    SpriteAnim {
                        size: anim.size.to_f32(),
                        frames: anim.frames,
                        sprites: {
                            anim.cels
                                .into_iter()
                                .map(|cel| {
                                    Sprite::new_ext(
                                        texture.clone(),
                                        cel.rect.to_f32(),
                                        cel.off.to_f32(),
                                        cel.size.to_f32(),
                                    )
                                })
                                .collect()
                        },
                        tags: anim.tags,
                        layers: anim.layers,
                    }
                })
            })
            .collect();

        AtlasGraphics {
            texture,
            sprites,
            sheets,
            fonts,
            patches,
            anims,
        }
    }
}
