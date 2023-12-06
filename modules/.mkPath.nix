# not a module
{
  pkgs,
  lib,
}: let
  inherit
    (lib)
    mkOption
    types
    mdDoc
    ;

  pathModule = defaultPrefix: {
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
        description = mdDoc "Source path";
      };

      destination = mkOption {
        type = types.str;
        description = mdDoc "Destination path";
        default = name;
      };

      prefix = mkOption {
        type = types.str;
        description = mdDoc "Prefix to prepend to destination path";
        default = defaultPrefix;
      };
    };
  };

  mkPathOption = defaultPrefix:
    mkOption {
      default = {};
      type = types.attrsOf (types.submodule (pathModule defaultPrefix));
    };

  mkPathConfig = cfg: cfgName: let
    pathsList = lib.attrValues cfg;
  in {
    static.derivations.${cfgName} = pkgs.runCommand "am-static-${cfgName}" {} ''
      trap "set +x" ERR
      set -x
      mkdir -p $out

      ${lib.concatMapStringsSep "\n" (value: ''
          destFile="$out/${value.prefix}${value.destination}"
          destDir="$(dirname "$destFile")"
          mkdir -p "$destDir"
          ln -vsfT ${value.source} "$destFile"
        '')
        pathsList}

      set +x
    '';

    dag.nodes = lib.mapAttrs' (name: value:
      lib.nameValuePair "am-${cfgName}-${name}" {
        command = [
          (lib.getExe (pkgs.writeShellScriptBin "am-path-activate" ''
            destFile="$AM_ROOT/$1"
            destDir="$(dirname "$destFile")"
            if [[ ! -d "$destDir" ]]; then
              mkdir -pv "$destDir"
            fi

            ln -vsfT "$AM_STATIC/$1" "$destFile"
          ''))
          "${value.prefix}${value.destination}"
        ];
      })
    cfg;
  };
in {
  inherit mkPathConfig mkPathOption;
}
