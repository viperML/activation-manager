let
  mod =
    { pkgs, config, ... }:
    {
      home.file."foo" = {
        target = "hello";
      };
    };
in
import ../nix/home {
  modules = [ mod ];
}
