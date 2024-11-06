local love = require("love")
local Ground = require("lib.ground").Ground
local Player = require("lib.player").Player
local utils = require("utils.index")
local Animation = require("utils.animation").Animation

local CAT_WALKING_PATH = "assets/cat-walking.png"

---@class Player
local player = {}

---@class Ground
local ground = {}

---@class Animation
local animation = {}

---@diagnostic disable-next-line
function love.load()
    local width, height = love.window.getMode()
    local catImage = love.graphics.newImage(CAT_WALKING_PATH)
    animation = Animation:new(catImage, 64, 64, 1)
    ground = Ground:new(width, height)
    player = Player:new(width / 2, ground.y)
    local skyBlue = utils.colorFrom256(141, 224, 244)
    love.graphics.setBackgroundColor(skyBlue)
end

---@param dt number
---@diagnostic disable-next-line
function love.update(dt)
    player:movement(dt, ground.y)
    animation.currentTime = animation.currentTime + dt
    if animation.currentTime >= animation.duration then
        animation.currentTime = animation.currentTime - animation.duration
    end
end

---@diagnostic disable-next-line
function love.draw()
    ground:draw()
    player:draw()
    local spriteNum = math.floor(animation.currentTime / animation.duration * #animation.quads) + 1
    love.graphics.draw(animation.spriteSheet, animation.quads[spriteNum], 0, 0, 0, 4)
end
