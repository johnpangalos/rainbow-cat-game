local love = require("love")
local Ground = require("lib.ground").Ground
local Player = require("lib.player").Player
local utils = require("utils")

---@class Player
local player = {}

---@class Ground
local ground = {}

---@diagnostic disable-next-line
function love.load()
	local width, height = love.window.getMode()
	ground = Ground:new(width, height)
	player = Player:new(width / 2, ground.y)
	local skyBlue = utils.colorFrom256(141, 224, 244)
	love.graphics.setBackgroundColor(skyBlue)
end

---@param dt number
---@diagnostic disable-next-line
function love.update(dt)
	player:movement(dt, ground.y)
end

---@diagnostic disable-next-line
function love.draw()
	ground:draw()
	player:draw()
end
