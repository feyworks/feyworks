---@meta

---@alias BlendMode "normal"|"add"|"subtract"|"multiply"
---@alias Topology "triangles"|"lines"|"points"

---@class (exact) Draw: DrawMethods

---@class DrawClass : DrawMethods
local module = {}

---@class DrawMethods
local methods = {}

---Set the target surface and optionally clear it with a single color. If `None` is passed
---as the surface, the window will be drawn to. If `None` is passed as the clear color, then
---the surface will not be cleared, drawing will instead be appended to its current pixels.
---@param self Draw
---@param surface Surface?
---@param clear_color Color?
function methods.set_surface(self, surface, clear_color) end

---Set the target layer. For the most part you will be rendering to the default layer `0`
---but in rare cases you may want to use layers to improve render batching.
---@param self Draw
---@param layer integer
function methods.set_layer(self, layer) end

---Set the shader future drawing methods will use. If the shader is already in use, nothing
---will happen. If not, the shader will switch and all the new shader's parameters will be
---initialized with their default values.
---@param self DrawMethods
---@param shader Shader?
function methods.set_shader(self, shader) end

---Set an `i32` parameter.
---@param self Draw
---@param name string
---@param value integer
function methods.set_param_i32(self, name, value) end

---Set a `u32` parameter.
---@param self Draw
---@param name string
---@param value integer
function methods.set_param_u32(self, name, value) end

---Set an `f32` parameter.
---@param self Draw
---@param name string
---@param value number
function methods.set_param_f32(self, name, value) end

---Set a `vec2f` parameter.
---@param self Draw
---@param name string
---@param value Vec2
function methods.set_param_vec2(self, name, value) end

---Set a `vec3f` parameter.
---@param self Draw
---@param name string
---@param value Vec3
function methods.set_param_vec3(self, name, value) end

---Set a `vec4f` parameter.
---@param self Draw
---@param name string
---@param value Vec4
function methods.set_param_vec4(self, name, value) end

---Set a `mat2f` parameter.
---@param self Draw
---@param name string
---@param value Mat2
function methods.set_param_mat2(self, name, value) end

---Set a `mat3f` parameter.
---@param self Draw
---@param name string
---@param value Mat3
function methods.set_param_mat3(self, name, value) end

---Set a `mat4f` parameter.
---@param self Draw
---@param name string
---@param value Mat4
function methods.set_param_mat4(self, name, value) end

---Set a `texture_2d<f32>` parameter.
---@param self Draw
---@param name string
---@param value Texture
function methods.set_param_texture(self, name, value) end

---Set a `sampler` parameter.
---@param self Draw
---@param name string
---@param value Sampler
function methods.set_param_sampler(self, name, value) end

---Set the view matrix.
---@param self Draw
---@param value Mat4
function methods.set_view_matrix(self, value) end

---The current main sampler.
---@param self Draw
---@return Sampler
---@nodiscard
function methods.main_sampler(self) end

---Set the main sampler.
---@param self Draw
---@param value Sampler
function methods.set_main_sampler(self, value) end

---The current blend mode.
---@param self Draw
---@return Sampler
---@nodiscard
function methods.blend_mode(self) end

---Set the blend mode.
---@param self Draw
---@param value BlendMode
function methods.set_blend_mode(self, value) end

---The current clip rectangle.
---@param self Draw
---@return Rect
---@nodiscard
function methods.clip_rect(self) end

---Set the clip rectangle.
---@param self Draw
---@param value Rect
function methods.set_clip_rect(self, value) end

---The current transform.
---@param self Draw
---@return Affine2
---@nodiscard
function methods.transform(self) end

---Concatenate and push a transform to the stack.
---@param self Draw
---@param matrix Affine2
function methods.push_transform(self, matrix) end

---Push a new transform value to the top of the stack.
---@param self Draw
---@param matrix Affine2
function methods.push_new_transform(self, matrix) end

---Set the value of the top transform.
---@param self Draw
---@param matrix Affine2
function methods.set_transform(self, matrix) end

---Concatenate and push a translation matrix.
---@param self Draw
---@param amount Vec2
function methods.push_translation(self, amount) end

---Concatenate and push a rotation matrix.
---@param self Draw
---@param radians number
function methods.push_rotation(self, radians) end

---Concatenate and push a scaling matrix.
---@param self Draw
---@param scale Vec2|number
function methods.push_scale(self, scale) end

---Concatenate and push a translation/rotation/scaling matrix.
---@param self Draw
---@param translation Vec2
---@param rotation number
---@param scale Vec2|number
function methods.push_trs(self, translation, rotation, scale) end

---Pop a transform off the top of the stack.
---@param self Draw
function methods.pop_transform(self) end

---Pop a number of transforms off the top of the stack.
---@param self Draw
---@param count integer
function methods.pop_transforms(self, count) end

---Draw a quad filled with a texture.
---@param self Draw
---@param texture Texture
---@param quad Quad
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function methods.textured_quad(self, texture, quad, color, mode, flip_x, flip_y) end

---Draw a texture with the top-left at the provided position.
---@param texture Texture
---@param pos Vec2
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function methods.texture_at(self, texture, pos, color, mode, flip_x, flip_y) end

---Draw a single point.
---@param self Draw
---@param point Vec2
---@param color Color
function methods.point(self, point, color) end

---Draw a single point.
---@param self Draw
---@param x number
---@param y number
---@param color Color
function methods.point(self, x, y, color) end

---Draw a set of points.
---@param self Draw
---@param points Vec2[]
---@param color Color
function methods.points(self, points, color) end

---Draw a line.
---@param self Draw
---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param color Color
function methods.line(self, x1, y1, x2, y2, color) end

---Draw a line.
---@param self Draw
---@param from Vec2
---@param to Vec2
---@param color Color
function methods.line(self, from, to, color) end

---Draw a line.
---@param self Draw
---@param line Line
---@param color Color
function methods.line(self, line, color) end

---Draw lines connecting the series of points into a chain, optionally looping to the start.
---@param self Draw
---@param points Vec2[]
---@param color Color
---@param loops boolean
function methods.lines(self, points, color, loops) end

---Draw a filled triangle.
---@param self Draw
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param color Color
function methods.triangle(self, a, b, c, color) end

---Draw a filled triangle.
---@param self Draw
---@param tri Triangle
---@param color Color
function methods.triangle(self, tri, color) end

---Draw a triangle outline.
---@param self Draw
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param color Color
function methods.triangle_outline(self, a, b, c, color) end

---Draw a triangle outline.
---@param self Draw
---@param tri Triangle
---@param color Color
function methods.triangle_outline(self, tri, color) end

---Draw a filled quad.
---@param self Draw
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param d Vec2
---@param color Color
function methods.quad(self, a, b, c, d, color) end

---Draw a filled quad.
---@param self Draw
---@param quad Quad
---@param color Color
function methods.quad(self, quad, color) end

---Draw a quad outline.
---@param self Draw
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param d Vec2
---@param color Color
function methods.quad_outline(self, a, b, c, d, color) end

---Draw a quad outline.
---@param self Draw
---@param quad Quad
---@param color Color
function methods.quad_outline(self, quad, color) end

---Draw a filled rectangle.
---@param self Draw
---@param x number
---@param y number
---@param w number
---@param h number
---@param color Color
function methods.rect(self, x, y, w, h, color) end

---Draw a filled rectangle.
---@param self Draw
---@param rect Rect
---@param color Color
function methods.rect(self, rect, color) end

---Draw a rectangle outline.
---@param self Draw
---@param x number
---@param y number
---@param w number
---@param h number
---@param color Color
function methods.rect_outline(self, x, y, w, h, color) end

---Draw a rectangle outline.
---@param self Draw
---@param rect Rect
---@param color Color
function methods.rect_outline(self, rect, color) end

---Draw a filled polygon.
---@param self Draw
---@param poly Polygon
---@param color Color
function methods.polygon(self, poly, color) end

---Draw a polygon outline.
---@param self Draw
---@param poly Polygon
---@param color Color
function methods.polygon_outline(self, poly, color) end

---Draw a filled circle.
---@param self Draw
---@param x number
---@param y number
---@param radius number
---@param color Color
---@param seg_count integer?
function methods.circle(self, x, y, radius, color, seg_count) end

---Draw a filled circle.
---@param self Draw
---@param center Vec2
---@param radius number
---@param color Color
---@param seg_count integer?
function methods.circle(self, center, radius, color, seg_count) end

---Draw a filled circle.
---@param self Draw
---@param circ Circle
---@param color Color
---@param seg_count integer?
function methods.circle(self, circ, color, seg_count) end

---Draw a circle outline.
---@param self Draw
---@param x number
---@param y number
---@param radius number
---@param color Color
---@param seg_count integer?
function methods.circle_outline(self, x, y, radius, color, seg_count) end

---Draw a circle outline.
---@param self Draw
---@param center Vec2
---@param radius number
---@param color Color
---@param seg_count integer?
function methods.circle_outline(self, center, radius, color, seg_count) end

---Draw a circle outline.
---@param self Draw
---@param circ Circle
---@param color Color
---@param seg_count integer?
function methods.circle_outline(self, circ, color, seg_count) end

---Draw a subtexture.
---@param sub SubTexture
---@param dst Quad
---@param color Color?
---@param mode ColorMode?
function methods.subtexture(sub, dst, color, mode) end

---Draw a subtexture.
---@param sub SubTexture
---@param pos Vec2
---@param color Color?
---@param mode ColorMode?
function methods.subtexture_at(sub, pos, color, mode) end

---Draw text with the provided font and size.
---@param self DrawMethods
---@param font Font
---@param text string
---@param size number?
---@param pos Vec2
---@param color Color
function methods.text(self, font, text, size, pos, color) end

---Draw a custom set of vertices & indices.
---@param self Draw
---@param texture Texture?
---@param topology Topology
---@param vertices Vertex[]
---@param indices integer[]
function methods.custom(self, texture, topology, vertices, indices) end

---Draw the provided vertex & index buffers.
---@param self Draw
---@param texture Texture?
---@param topology Topology
---@param vertices VertexBuffer
---@param indices IndexBuffer
function methods.buffers(self, texture, topology, vertices, indices) end

return module