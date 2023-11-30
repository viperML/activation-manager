{
  lib,
  config,
  pkgs,
  ...
}: let
  inherit
    (lib)
    mkOption
    types
    mdDoc
    ;

  fileModule = {
    name,
    config,
    ...
  }: {
    options = {
      enable = mkOption {
        type = types.bool;
        default = true;
        description = mdDoc "Whether this path is used or not";
      };

      source = mkOption {
        type = types.path;
        description = mdDoc "Path of the source";
      };
    };
  };
in {
  imports = [
    ./dag.nix
  ];

  options = {
    path = mkOption {
      default = {};
      type = types.attrsOf (types.submodule fileModule);
    };
  };

  config = let
    allPaths = lib.attrsToList config.path;
  in {
    static.derivations.path = pkgs.runCommandLocal "activation-manager-static-path" {} ''
      trap "set +x" ERR
      set -x
      mkdir -p $out
      pushd $out
      ${lib.concatMapStringsSep "\n" ({
        name,
        value,
      }: "ln -vsT ${value.source} ${name}")
      allPaths}
      popd
      set +x
    '';

    dag.nodes."path" = {
      exec = pkgs.writeShellScript "activation-manager-path" ''
        ${lib.concatMapStringsSep "\n" ({
          name,
          value,
        }: "ln -vsT ${config.static.derivations.path}/${name} $ROOT/${name} ")
        allPaths}
      '';
    };
  };
}
