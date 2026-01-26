---@meta

---@class (exact) Sprite: SpriteMethods

---@class SpriteClass: SpriteMethods
local module = {}

---@class SpriteMethods
local methods = {}

---Create a new sprite from the rectangular sub-region of a texture's pixels.
---You can also provide a rendering offset and virtual size for the subtexture.
---@param texture Texture
---@param rect Rect
---@param offset Vec2?
---@param size Vec2?
---@return Sprite
---@nodiscard
function module.new(texture, rect, offset, size) end

---The sprite's source rectangle.
---@param self Sprite
---@return Rect
---@nodiscard
function methods.rect(self) end

---The sprite's render offset.
---@param self Sprite
---@return Vec2
---@nodiscard
function methods.offset(self) end

---The sprite's virtual size.
---@param self Sprite
---@return Vec2
---@nodiscard
function methods.size(self) end

---The sprite's texture coordinates.
---@param self Sprite
---@return Vec2
---@return Vec2
---@return Vec2
---@return Vec2
---@nodiscard
function methods.coords(self) end

---Draw this sprite at the provided position.
---@param self Sprite
---@param pos Vec2
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function methods.draw(self, pos, color, mode, flip_x, flip_y) end

return module