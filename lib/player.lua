local love = require("love")
local constants = require("constants")
local utils = require("utils")

local PLAYER_IMAGE_PATH = "assets/rainbow-cat-32x32.png"

---@class Player
---@field x number
---@field y number
---@field width integer
---@field img love.Image
---@field yVelocity number
---@field jumpHeight integer
---@field gravity integer
---@field facing "left" | "right"
local Player = {}

---@class Player
---@param x number
---@param y number
---@return Player
function Player:new(x, y)
	local obj = {
		x = x,
		y = y,
		img = love.graphics.newImage(PLAYER_IMAGE_PATH),
		width = 32,
		yVelocity = 0,
		jumpHeight = -300,
		gravity = -500,
		facing = "right",
	}
	self.__index = self
	return setmetatable(obj, self)
end

---@class Player
---@return nil
function Player:draw()
	local xScale = self.facing == "left" and 1 or -1
	love.graphics.draw(self.img, self.x, self.y, 0, xScale, 1, 0, self.width)
end

---@class Player
---@param dt number
---@param yGround number
---@return nil
function Player:movement(dt, yGround)
	local width = love.window.getMode()

	if self.x > 0 and (utils.isLeftDown()) then
		self.x = self.x - dt * constants.PLAYER_SPEED
		self.facing = "left"
	end

	if self.x < width - self.width and (utils.isRightDown()) then
		self.x = self.x + dt * constants.PLAYER_SPEED
		self.facing = "right"
	end

	if love.keyboard.isDown("space") and self.yVelocity == 0 then
		self.yVelocity = self.jumpHeight
	end

	if self.yVelocity ~= 0 then
		self.y = self.y + self.yVelocity * dt
		self.yVelocity = self.yVelocity - self.gravity * dt
	end

	if self.y > yGround then
		self.yVelocity = 0
		self.y = yGround
	end
end

return { Player = Player }
