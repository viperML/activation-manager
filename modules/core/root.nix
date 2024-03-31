{ config, lib, ... }:
let
  inherit (lib) mkOption types;
in
{
  options = {
    root.location = mkOption {
      type = types.either (types.listOf types.str) (types.path);
      description = "Root directory location. Either a command that outputs to stdout, or an absolute path";
      default = [
        "printenv"
        "HOME"
      ];
    };
  };

  config = { };
}
