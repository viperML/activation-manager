{
  pkgs,
  config,
  lib,
  ...
}:
let
  inherit (lib) mkOption types;

  fileSubmodule =
    { name, ... }:
    {
      options = {
        target = mkOption {
          type = types.str;
        };
        link = mkOption {
          type = types.str;
          default = name;
        };
      };
    };
in
{
  options = {
    build = {
      package = mkOption {
        type = types.package;
        default = pkgs.callPackage ../package.nix { };
      };

      manifest = mkOption {
        type = types.package;
      };

      bundle = mkOption {
        type = types.package;
      };
    };

    home = {
      file = mkOption {
        default = { };
        type = types.attrsOf (types.submodule fileSubmodule);
      };
    };

  };

  config = {
    build.manifest =
      pkgs.writeText "manifest.lua"
        # lua
        ''
          local am = require("am")
          local os = require("os")
          local home = os.getenv("HOME")

          local static = home .. "/.local/state/activation-manager/static"

          ${
            config.home.file
            |> builtins.attrValues
            |> (map (node:
            # lua
            ''
              am.file {
                link = home .. "/${node.link}",
                target = static .. "/${node.link}",
              }

              am.file {
                link = static .. "/${node.link}",
                target = home .. "/${node.target}",
              }
            ''))
            |> builtins.concatStringsSep "\n"
          }
        '';

    build.bundle =
      pkgs.runCommandLocal "am-bundle"
        {
          nativeBuildInputs = [ pkgs.makeWrapper ];
        }
        ''
          mkdir -p $out/bin
          makeWrapper ${lib.getExe config.build.package} $out/bin/activate \
            --append-flags ${config.build.manifest}
        '';
  };

}
