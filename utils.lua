---@param red integer
---@param green integer
---@param blue integer
---@return table
local function colorFrom256(red, green, blue)
	return { red / 256, green / 256, blue / 256 }
end

---@return boolean
local function isLeftDown()
	return love.keyboard.isDown("left") or love.keyboard.isDown("a")
end
---
---@return boolean
local function isRightDown()
	return love.keyboard.isDown("right") or love.keyboard.isDown("d")
end

return {
	colorFrom256 = colorFrom256,
	isLeftDown = isLeftDown,
	isRightDown = isRightDown,
}
