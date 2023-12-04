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

      exec = mkOption {
        type = types.nullOr types.package;
        default = null;
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
