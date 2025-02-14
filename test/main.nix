let
  mod =
    { pkgs, config, ... }:
    {
      home.file."foo" = {
        target = "hello";
      };
      dconf.settings = {
        "/org/gnome/desktop/peripherals/mouse/accel-profile" = "flat";
      };
    };
in
import ../nix/home {
  modules = [ mod ];
}
