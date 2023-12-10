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

      recursive = mkOption {
        type = types.bool;
        description = mdDoc ''
          If the source is a directory, try to walk it linking all the inmediate files or symlink leaves.

          Useful for keeping part of a folder mutable (e.g. ~/.config/systemd).
        '';
        default = false;
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
      lib.nameValuePair "${cfgName}-${name}" {
        after = ["static"];
        command = [
          (
            if !value.recursive
            then
              (lib.getExe (pkgs.writeShellScriptBin "am-path-activate" ''
                set -eu
                destPath="$AM_ROOT/$1"
                destDir="$(dirname "$destPath")"
                if [[ ! -d "$destDir" ]]; then
                  mkdir -pv "$destDir"
                fi

                ln -vsfT "$AM_STATIC/$1" "$destPath"
              ''))
            else
              (lib.getExe (pkgs.writeShellScriptBin "am-path-activate-recursive" ''
                set -eu
                shopt -s nullglob
                shopt -s globstar
                for sourcePath in "$AM_STATIC/$1"/**/*; do
                  # surely there's a better way to only glob files or links
                  if [[ -d "$sourcePath" ]]; then
                    continue
                  fi
                  # echo "sourcePath=$sourcePath"

                  prefix="''${sourcePath#$AM_STATIC}"
                  destPath="$AM_ROOT$prefix"
                  destDir="$(dirname "$destPath")"
                  # echo destDir="$destDir"
                  if [[ ! -d "$destDir" ]]; then
                    # echo $ mkdir -pv "$destDir"
                    mkdir -pv "$destDir"
                  fi

                  # echo $ ln -vsfT "$sourcePath" "$destPath"
                  ln -vsfT "$sourcePath" "$destPath"
                done
              ''))
          )
          "${value.prefix}${value.destination}"
        ];
      })
    cfg;
  };
in {
  inherit mkPathConfig mkPathOption;
}
