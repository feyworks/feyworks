---@meta

---A 2D axis-aligned rectangle. Most of the methods for this struct assume that the rectangle has a positive
---width and height, so rectangles where T is signed may yield incorrect values for negative-sized instances.
---@class (exact) RectI: RectIMethods
---@field x integer
---@field y integer
---@field w integer
---@field h integer
---@operator mul(Vec2I|integer): RectI
---@operator div(Vec2I|integer): RectI

---@class RectIClass : RectIMethods
---@overload fun(x: integer, y: integer, w: integer, h: integer): RectI
---@overload fun(w: integer, h: integer): RectI
local module = {}

---@class RectIMethods: Shape
local methods = {}

---Create a new rect.
---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@return RectI
---@nodiscard
function module.new(x, y, w, h) end

---Create a new rect.
---@param w integer
---@param h integer
---@return RectI
---@nodiscard
function module.new(w, h) end

---Checks if two values are equal.
---@param self RectI
---@param other RectI
---@return boolean
---@nodiscard
function methods.eq(self, other) end

---Returns a temporary copy of this value.
---@param self RectI
---@return RectI
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self RectI
---@return RectI
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self RectI
---@return RectI
---@nodiscard
function methods.box_clone(self) end

---Set the rectangle's value.
---@param self RectI
---@param x integer
---@param y integer
---@param w integer
---@param h integer
function methods.set(self, x, y, w, h) end

---Set the rectangle's value.
---@param self RectI
---@param rect RectI
function methods.set_to(self, rect) end

---Returns the width and height of the rectangle.
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.size(self) end

---The left edge of the rectangle (`x`).
---@param self RectI
---@return integer
---@nodiscard
function methods.left(self) end

---The right edge of the rectangle (`x + w`).
---@param self RectI
---@return integer
---@nodiscard
function methods.right(self) end

---The top edge of the rectangle (`y`).
---@param self RectI
---@return integer
---@nodiscard
function methods.top(self) end

---The bottom edge of the rectangle (`y + h`).
---@param self RectI
---@return integer
---@nodiscard
function methods.bottom(self) end

---The top-left point of the rectangle (`x, y`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.top_left(self) end

---The top-right point of the rectangle (`x + w, y`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.top_right(self) end

---The bottom-right point of the rectangle (`x + w, y + h`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.bottom_right(self) end

---The bottom-left point of the rectangle (`x, y + h`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.bottom_left(self) end

---All 4 corners of the rectangle.
---@param self RectI
---@return Vec2I top_left
---@return Vec2I top_right
---@return Vec2I bottom_right
---@return Vec2I bottom_left
---@nodiscard
function methods.corners(self) end

---The top-center point of the rectangle (`x + w / 2, y`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.top_center(self) end

---The bottom-center point of the rectangle (`x + w / 2, y + h`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.bottom_center(self) end

---The left-center point of the rectangle (`x, y + h / 2`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.left_center(self) end

---The right-center point of the rectangle (`x + w, y + h / 2`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.right_center(self) end

---The horizontal center of the rectangle (`x + w / 2`).
---@param self RectI
---@return integer
---@nodiscard
function methods.center_x(self) end

---The vertical center of the rectangle (`y + h / 2`).
---@param self RectI
---@return integer
---@nodiscard
function methods.center_y(self) end

---The center of the rectangle (`x + w / 2, y + h / 2`).
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.center(self) end

---The minimum x-position of the rectangle (`min(x, x + w)`).
---@param self RectI
---@return integer
---@nodiscard
function methods.min_x(self) end

---The minimum y-position of the rectangle (`min(y, y + h)`).
---@param self RectI
---@return integer
---@nodiscard
function methods.min_y(self) end

---The minimum point of the rectangle (`min(x, x + w), min(y, y + h)`)
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.min_pos(self) end

---The maximum x-position of the rectangle (`max(x, x + w)`).
---@param self RectI
---@return integer
---@nodiscard
function methods.max_x(self) end

---The maximum y-position of the rectangle (`max(y, y + h)`).
---@param self RectI
---@return integer
---@nodiscard
function methods.max_y(self) end

---The maximum point of the rectangle (`max(x, x + w), max(y, y + h)`)
---@param self RectI
---@return integer
---@return integer
---@nodiscard
function methods.max_pos(self) end

---The right edge of the rectangle.
---@param self RectI
---@return Line
---@nodiscard
function methods.right_edge(self) end

---The left edge of the rectangle.
---@param self RectI
---@return Line
---@nodiscard
function methods.left_edge(self) end

---The top edge of the rectangle.
---@param self RectI
---@return Line
---@nodiscard
function methods.top_edge(self) end

---The bottom edge of the rectangle.
---@param self RectI
---@return Line
---@nodiscard
function methods.bottom_edge(self) end

---All 4 edges of the rectangle.
---@param self RectI
---@return Line right
---@return Line bottom
---@return Line left
---@return Line top
---@nodiscard
function methods.edges(self) end

---Area of the rectangle.
---@param self RectI
---@return integer
---@nodiscard
function methods.area(self) end

---Perimeter of the rectangle.
---@param self RectI
---@return integer
---@nodiscard
function methods.perimeter(self) end

---Returns `true` if the rectangle is inside this one.
---@param self RectI
---@param inner RectI
---@return boolean
---@nodiscard
function methods.contains_rect(self, inner) end

---Inflate the rectangle by the amount
---@param self RectI
---@param w integer
---@param h integer?
---@return RectI
---@nodiscard
function methods.inflate(self, w, h) end

---If the two rectangles overlap, returns the overlapping region.
---@param self RectI
---@param other RectI
---@return RectI?
---@nodiscard
function methods.overlap(self, other) end

---Returns a rectangle that exactly encoses the two rectangles.
---@param self RectI
---@param other RectI
---@return RectI
---@nodiscard
function methods.conflate(self, other) end

---Clamps the point within the ractangle.
---@param self RectI
---@param p Vec2I
---@return Vec2I
---@nodiscard
---@overload fun(self: RectI, x: integer, y: integer): integer, integer
function methods.clamp_inside(self, p) end

---Returns `true` if both `w >= 0` and `h >= 0`.
---@param self RectI
---@return boolean
---@nodiscard
function methods.is_positive(self) end

---Returns a non-negative sized equivalent of this rectangle. For example, `(10, 10, -4, -4)` will
---be converted to `(6, 6, 4, 4)`.
---@param self RectI
---@return RectI
---@nodiscard
function methods.non_neg(self) end

---Translates the rectangle by the amount.
---@param self RectI
---@param amount Vec2I
---@return RectI
---@nodiscard
function methods.translate(self, amount) end

---If the point is contained within the rectangle.
---@param self RectI
---@param p Vec2I
---@return boolean
---@nodiscard
function methods.contains(self, p) end

---If this shape is convex.
---@param self RectI
---@return boolean
---@nodiscard
function methods.is_convex(self) end

return module
