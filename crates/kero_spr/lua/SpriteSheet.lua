---@meta

---A sheet of sprite tiles.
---@class (exact) SpriteSheet: SpriteSheetMethods

---@class SpriteSheetClass: SpriteSheetMethods
local module = {}

---@class SpriteSheetMethods
local methods = {}

---Draw a tile from the sheet.
---@param self SpriteSheet
---@param col integer
---@param row integer
---@param pos Vec2
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function methods.draw_tile(self, col, row, pos, color, mode, flip_x, flip_y) end

return module