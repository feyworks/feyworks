---@meta

---@class (exact) SpritePatch: SpritePatchMethods

---@class SpritePatchClass: SpritePatchMethods
module = {}

---@class SpritePatchMethods
local methods = {}

---Create a new patch, using the `inner` rectangle to split the
---`outer` rectangle into 9 sub-rectangles.
---@param texture Texture
---@param outer Rect
---@param inner Rect
---@return SpritePatch
---@nodiscard
function module.new(texture, outer, inner) end

---Draw the patch, covering the provided rectangular region.
---@param self SpritePatch
---@param rect Rect
---@param color Color?
---@param mode ColorMode?
function methods.draw(self, rect, color, mode) end

return module
