local am = require("activation-manager")
am.debug("Hello")

local nodes = {}

table.insert(nodes, {
  "static",
})

table.insert(nodes, {
  "static-env",
  function()
    am.debug("Running static-env")
  end,
  after = {"static"},
})

return nodes
