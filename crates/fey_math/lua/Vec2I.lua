---@meta

---A 2-dimensional vector.
---@class (exact) Vec2I: Vec2IMethods
---@field x integer
---@field y integer
---@operator add(Vec2I): Vec2I
---@operator sub(Vec2I): Vec2I
---@operator mul(Vec2I|integer): Vec2I
---@operator div(Vec2I|integer): Vec2I
---@operator unm: Vec2I

---@class Vec2IClass : Vec2IMethods
---@overload fun(x: integer, y: integer): Vec2I
local module = {}

---@class Vec2IMethods
local methods = {}

---`(0, 0)`
---@return Vec2I
---@nodiscard
function module.zero() end

---`(1, 1)`
---@return Vec2I
---@nodiscard
function module.one() end

---`(1, 0)`
---@return Vec2I
---@nodiscard
function module.x_axis() end

---`(0, 1)`
---@return Vec2I
---@nodiscard
function module.y_axis() end

---`(1, 0)`
---@return Vec2I
---@nodiscard
function module.right() end

---`(-1, 0)`
---@return Vec2I
---@nodiscard
function module.left() end

---`(0, 1)`
---@return Vec2I
---@nodiscard
function module.down() end

---`(0, -1)`
---@return Vec2I
---@nodiscard
function module.up() end

---`(1, 0)`
---@return Vec2I
---@nodiscard
function module.east() end

---`(0, 1)`
---@return Vec2I
---@nodiscard
function module.south() end

---`(-1, 0)`
---@return Vec2I
---@nodiscard
function module.west() end

---`(0, -1)`
---@return Vec2I
---@nodiscard
function module.north() end

---Create a new vector.
---@param x integer
---@param y integer
---@return Vec2I
---@nodiscard
function module.new(x, y) end

---Create a new vector `(val, val)`.
---@param val integer
---@return Vec2I
---@nodiscard
function module.splat(val) end

---Checks if two values are equal.
---@param self Vec2I
---@param other Vec2I
---@return boolean
---@nodiscard
function methods.eq(self, other) end

---Returns a temporary copy of this value.
---@param self Vec2I
---@return Vec2I
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Vec2I
---@return Vec2I
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Vec2I
---@return Vec2I
---@nodiscard
function methods.box_clone(self) end

---Set the vector's coordinates.
---@param self Vec2I
---@param x integer
---@param y integer
function methods.set(self, x, y) end

---Set the vector's coordinates.
---@param self Vec2I
---@param value Vec2I
function methods.set_to(self, value) end

---Returns a copy of this vector with absolute value of all components.
---@param self Vec2I
---@return Vec2I
---@nodiscard
function methods.abs(self) end

---The vector's angle in radians.
---@param self Vec2I
---@return number
---@nodiscard
function methods.angle(self) end

---Clamp's the vector's components between the components of `min` and `max`.
---@param self Vec2I
---@param min Vec2I
---@param max Vec2I
---@return Vec2I
---@nodiscard
function methods.clamp(self, min, max) end

---Returns the cross product of two vectors.
---@param self Vec2I
---@param other Vec2I
---@return integer
---@nodiscard
function methods.cross(self, other) end

---Returns the distance between two points.
---@param self Vec2I
---@param point Vec2I
---@return number
---@nodiscard
function methods.dist(self, point) end

---Returns the manhattan distance between two points.
---@param self Vec2I
---@param point Vec2I
---@return integer
---@nodiscard
function methods.man_dist(self, point) end

---Returns the dot product of two vectors.
---@param self Vec2I
---@param other Vec2I
---@return integer
---@nodiscard
function methods.dot(self, other) end

---Returns `true` if the vector is equal to `(0, 0)`.
---@return boolean
---@nodiscard
function methods.is_zero(self) end

---Length of the vector.
---@param self Vec2I
---@return number
---@nodiscard
function methods.len(self) end

---Manhattan length of the vector.
---@param self Vec2I
---@return integer
---@nodiscard
function methods.len(self) end

---Returns a vector with components set to the largest values in the arguments.
---@param self Vec2I
---@param ... Vec2I
---@return Vec2I
---@nodiscard
function methods.max(self, ...) end

---Returns a vector with components set to the smallest values in the arguments.
---@param self Vec2I
---@param ... Vec2I
---@return Vec2I
---@nodiscard
function methods.min(self, ...) end

---Normalizes the vector.
---@param self Vec2I
---@return Vec2
---@nodiscard
function methods.norm(self) end

---Returns a copy of this vector with all components signed (set to `1`, `-1`, or `0`).
---@param self Vec2I
---@return Vec2I
---@nodiscard
function methods.sign(self) end

---Returns the square between two points.
---@param self Vec2I
---@param point Vec2I
---@return Vec2I
---@nodiscard
function methods.sqr_dist(self, point) end

---Squared length of the vector.
---@param self Vec2I
---@return integer
---@nodiscard
function methods.sqr_len(self) end

---Rotates the vector 90° counter-clockwise.
---@param self Vec2I
---@return Vec2I
---@nodiscard
function methods.turn_left(self) end

---Rotates the vector 90° clockwise.
---@param self Vec2I
---@return Vec2I
---@nodiscard
function methods.turn_right(self) end

---Changes the length of the vector without changing its direction.
---@param self Vec2I
---@param new_len number
---@return Vec2
---@nodiscard
function methods.with_len(self, new_len) end

---Replaces the `x` component of the vector.
---@param self Vec2I
---@param x integer
---@return Vec2I
---@nodiscard
function methods.with_x(self, x) end

---Replaces the `y` component of the vector.
---@param self Vec2I
---@param y integer
---@return Vec2I
---@nodiscard
function methods.with_y(self, y) end

---Add a third dimension to this vector.
---@param self Vec2I
---@param z integer
---@return Vec3
---@nodiscard
function methods.with_z(self, z) end

---Swizzles the vector.
---@param self Vec2I
---@return Vec2I
---@nodiscard
function methods.yx(self) end

---Approach the target by the provided amount without overshooting.
---@param self Vec2I
---@param target Vec2I
---@param amount integer
---@return Vec2I
---@nodiscard
function methods.approach(self, target, amount) end

---Linear interpolation.
---@param self Vec2I
---@param target Vec2I
---@param t integer
---@return Vec2I
---@nodiscard
function methods.lerp(self, target, t) end

return module
