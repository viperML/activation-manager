local am = require("activation-manager")

am.debug("xd")

-- print("hello")

-- nodes.add {
--   name = "static",
--   before = {},
-- }
local nodes = am.Nodes()

nodes:add {
  name = "static",
}

nodes:add {
  name = "static-env",
  after = {"static"}
}

-- nodes.add {
--   name = "foo"
-- }

return nodes
