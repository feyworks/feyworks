local Keyboard = require "Keyboard"
local Key      = require "Key"
local Mouse    = require "Mouse"
local Vec2     = require "Vec2"
local Color    = require "Color"
local Draw     = require "Draw"

local Main     = {}

function Main:init()

end

function Main:update()
    if Keyboard.pressed(Key.SPACE) then
        print("SPACE!")
    end
end

function Main:render()
    local m = Mouse.pos()
    Draw.line(0, 0, m.x, m.y, Color.red())
end

return Main
