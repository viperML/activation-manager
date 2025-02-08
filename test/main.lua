local am = require("am")

am.file {
  from = "/nix/store/foo",
  to = "/run/user/10000/foo",
}

-- local fbar = am.file {
--   from = "/nix/store/bar",
--   to = ".config/bar",
-- }
--
-- local f = am.home .. "f"
--
-- local x = am.run {
--   command = { "touch", f },
--   after = { fbar },
-- }
--
-- am.run {
--   command = { "file", f },
--   after = { x },
--   hide = false,
-- }
--
-- am.systemd_user {
--   unit_file = "/nix/store/...service",
-- }
