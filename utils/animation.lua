---@class Animation 
---@field spriteSheet love.Image
---@field quads love.Quad[]
---@field duration number
---@field currentTime number
local Animation = {}

---@class Animation
---@param image love.Image
---@param width number
---@param height number
---@param duration number
function Animation:new(image, width, height, duration)
  local obj = {
    spriteSheet = image,
    duration = duration,
    currentTime = 0,
    quads = {},
  }
  for y = 0, obj.spriteSheet:getHeight() - height, height do
    for x = 0, obj.spriteSheet:getWidth() - width, width do
      table.insert(obj.quads, love.graphics.newQuad(x, y, width, height, obj.spriteSheet:getDimensions()))
    end
  end
  self.__index = self
  return setmetatable(obj, self)
end

return { Animation = Animation }
