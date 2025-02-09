let
  mod =
    { pkgs, config, ... }:
    {
      home.file."foo" = {
        from = "bar";
      };
    };
in
import ../nix/home {
  modules = [ mod ];
}
