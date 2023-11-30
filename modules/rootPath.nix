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
    rootPath = {
      command = mkOption {
        type = types.nullOr types.str;
        description = mdDoc "Command to run that outputs the root directory";
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
