local am = require("am")
local os = require("os")

local home = os.getenv("HOME")

local f1 = am.file {
  from = home .. "/foo",
  to = home .. "/bar",
}

print(f1)

