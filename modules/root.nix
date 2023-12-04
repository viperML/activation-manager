{
  config,
  lib,
  ...
}: let
  inherit
    (lib)
    mkOption
    types
    mdDoc
    ;
in {
  options = {
    root.location = {
      command = mkOption {
        type = types.nullOr types.str;
        description = mdDoc "Command that outputs the location to stdout";
        default = null;
      };

      absolute = mkOption {
        type = types.nullOr types.path;
        description = mdDoc "Absolute path that is the root directory";
        default = null;
      };
    };
  };

  config = {};
}
