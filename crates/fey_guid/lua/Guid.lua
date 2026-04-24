---@meta

---A globally unique identifier.
---@class Guid: GuidMethods

---@class GuidClass: GuidMethods
---@overload fun(): Guid
local module = {}

---@class GuidMethods
local methods = {}

---Generator a new ID.
---@return Guid
---@nodiscard
function module.new() end

---Returns a temporary copy of this value.
---@param self Guid
---@return Guid
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Guid
---@return Guid
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Guid
---@return Guid
---@nodiscard
function methods.box_clone(self) end

return module