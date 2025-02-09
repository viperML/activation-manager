local am = require("am")
local os = require("os")

local home = os.getenv("HOME")



---@type string
local f1 = am.file {
  id = "f1",
  from = home .. "/foo",
  to = home .. "/bar",
}

