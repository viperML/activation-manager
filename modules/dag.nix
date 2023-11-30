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
      before = mkOption {
        type = types.listOf types.str;
        default = [];
      };

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

    manifest = mkOption {
      type = types.package;
      description = mdDoc "Resulting manifest of all entries";
    };
  };

  config.dag = {
    manifest = pkgs.writers.writeJSON "activation-manager-manifest.json" {
      version = "0";
      inherit (config) rootPath;
      nodes = builtins.attrValues config.dag.nodes;
    };
  };
}
