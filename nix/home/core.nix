{
  pkgs,
  config,
  options,
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
          description = "FIXME";
        };
        link = mkOption {
          type = types.str;
          default = name;
          description = "FIXME";
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
        description = "FIXME";
      };

      manifest = mkOption {
        type = types.package;
        description = "FIXME";
      };

      bundle = mkOption {
        type = types.package;
        description = "FIXME";
      };

      optionsDoc = mkOption {
        type = types.attrsOf types.package;
        visible = false;
        default = pkgs.nixosOptionsDoc {
          inherit options;
        };
        readOnly = true;
        description = "FIXME";
      };
    };

    home = {
      file = mkOption {
        default = { };
        type = types.attrsOf (types.submodule fileSubmodule);
        description = "FIXME";
      };
    };

    dconf = {
      settings = mkOption {
        default = { };
        type = types.attrsOf (types.anything);
        description = "FIXME";
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
          local static_new = static .. "-" .. os.time()

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
                link = static_new .. "/${node.link}",
                target = home .. "/${node.target}",
              }
            ''))
            |> builtins.concatStringsSep "\n"
          }

          am.file {
            link = static,
            target = static_new,
          }

          ${
            config.dconf.settings
            |> lib.attrsToList
            |> map (
              { name, value }:
              # lua
              "am.dconf ${
                lib.generators.toLua { } {
                  inherit value;
                  key = name;
                }
              }"
            )
            |> builtins.concatStringsSep "\n"
          }
        '';

    build.bundle =
      pkgs.runCommandLocal "am-bundle"
        {
          nativeBuildInputs = [ pkgs.makeWrapper ];
          meta.mainProgram = "activate";
        }
        ''
          mkdir -p $out/bin
          makeWrapper ${lib.getExe config.build.package} $out/bin/activate \
            --append-flags ${config.build.manifest}
        '';
  };

}
