local love = require("love")
local constants = require("constants")

---@class Player
---@field x number
---@field y number
---@field radius integer
local Player = {}

---@class Player
---@param radius integer
---@return Player
function Player:new(radius)
	local width, height = love.window.getMode()
	local obj = {
		radius = radius,
		x = width / 2 - radius / 2,
		y = height / 2 - radius / 2,
	}
	self.__index = self
	return setmetatable(obj, self)
end

---@class Player
function Player:draw()
	love.graphics.circle("fill", self.x, self.y, self.radius)
end

---@class Player
---@param dt number
function Player:movement(dt)
	if love.keyboard.isDown("up") or love.keyboard.isDown("w") then
		self.y = self.y - dt * constants.PLAYER_SPEED
	end
	if love.keyboard.isDown("down") or love.keyboard.isDown("s") then
		self.y = self.y + dt * constants.PLAYER_SPEED
	end
	if love.keyboard.isDown("left") or love.keyboard.isDown("a") then
		self.x = self.x - dt * constants.PLAYER_SPEED
	end
	if love.keyboard.isDown("right") or love.keyboard.isDown("d") then
		self.x = self.x + dt * constants.PLAYER_SPEED
	end
end

return { Player = Player }
