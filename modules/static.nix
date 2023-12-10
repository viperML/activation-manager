{
  config,
  lib,
  pkgs,
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
    static = {
      derivations = mkOption {
        type = types.attrsOf types.package;
        default = {};
        description = mdDoc "Derivations to merge into the static package";
      };

      result = mkOption {
        type = types.package;
        readOnly = true;
        description = mdDoc "Resulting merge of all static derivations";
      };

      location = {
        absolute = mkOption {
          type = types.nullOr types.path;
          default = null;
          description = mdDoc "Path to the static files location";
        };

        command = mkOption {
          type = types.nullOr (types.listOf types.str);
          description = mdDoc "Command to get the static files location. $AM_ROOT is available";
          default = null;
        };
      };
    };
  };

  config = {
    dag.nodes."static" = {
      command = [
        "sh"
        "-c"
        ''
          nix build ${config.static.result} --out-link "$AM_STATIC"
        ''
      ];
    };

    static = {
      result = pkgs.buildEnv {
        name = "am-static";
        paths = builtins.attrValues config.static.derivations;
      };
    };
  };
}
