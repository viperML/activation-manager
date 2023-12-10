{
  config,
  lib,
  pkgs,
  ...
}: let
  inherit
    (lib)
    mkOption
    mdDoc
    types
    ;

  dagNodeModule = {
    config,
    name,
    ...
  }: {
    options = {
      after = mkOption {
        type = types.listOf types.str;
        default = [];
      };

      before = mkOption {
        type = types.listOf types.str;
        default = [];
      };

      command = mkOption {
        type = types.nullOr (types.listOf types.str);
        default = null;
      };

      generatesNodes = mkOption {
        type = types.bool;
        default = false;
      };
    };
  };
in {
  options.dag = {
    nodes = mkOption {
      type = types.attrsOf (types.submodule dagNodeModule);
      description = mdDoc "Direct-acyclic graph entry";
      default = {};
    };
  };
}
