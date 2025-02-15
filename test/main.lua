local am = require("am")
local os = require("os")

local home = os.getenv("HOME")

---@type string
-- local f1 = am.file {
--   id = "f1",
--   from = home .. "/foo",
--   to = home .. "/bar",
-- }

am.dconf {
  key = "/org/gnome/desktop/peripherals/mouse/accel-profile",
  value = "flat",
  -- value = "flat",
}

am.exec {
  command = { "ls" },
}

am.exec {
  command = { "blhablah" },
}
