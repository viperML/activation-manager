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
        from = mkOption {
          type = types.str;
        };
        to = mkOption {
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
        type = types.str;
      };

      bundle = mkOption {
        type = types.package;
      };
    };

    home.file = mkOption {
      default = { };
      type = types.attrsOf (types.submodule fileSubmodule);
    };

  };

  config = {
    build.manifest = # lua
      ''
        local am = require("am")
        local os = require("os")
        local home = os.getenv("HOME")

        ${
          config.home.file
          |> builtins.attrValues
          |> (map (node:
          # lua
          ''
            am.file {
              from = home .. "/${node.from}",
              to = home .. "/${node.to}",
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
            --append-flags ${
              pkgs.writeText "manifest.lua" config.build.manifest
            }
        '';
  };

}
