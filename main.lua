local love = require("love")
local Player = require("lib.player").Player
local constants = require("constants")

local player = Player:new(constants.PLAYER_RADIUS)

---@param dt number
---@diagnostic disable-next-line
function love.update(dt)
	player:movement(dt)
end

---@diagnostic disable-next-line
function love.draw()
	player:draw()
end
