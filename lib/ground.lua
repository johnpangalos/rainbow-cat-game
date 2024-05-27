local love = require("love")
local constants = require("constants")

---@class Ground
---@field height number
---@field width number
---@field x number
---@field y number
local Ground = {}

---@class Ground
---@param width number
---@param height number
---@return Ground
function Ground:new(width, height)
	local obj = {
		width = width,
		height = height,
		x = 0,
		y = 2 * height / 3,
	}
	self.__index = self
	return setmetatable(obj, self)
end

---@class Ground
---@return nil
function Ground:draw()
	love.graphics.setColor(constants.GROUND_COLOR)
	love.graphics.rectangle("fill", self.x, self.y, self.width, self.height)
end

return { Ground = Ground }
