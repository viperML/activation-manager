{
  config,
  pkgs,
  lib,
  utils,
  ...
}: let
  inherit
    (lib)
    mkOption
    types
    mdDoc
    mapAttrs'
    nameValuePair
    ;

  inherit
    (utils)
    systemdUtils
    ;

  inherit
    (systemdUtils.lib)
    socketToUnit
    serviceToUnit
    ;

  cfg = config.systemd.user;
in {
  options = {
    systemd.user = {
      units = mkOption {
        default = {};
        type = systemdUtils.types.units;
      };

      services = mkOption {
        default = {};
        type = systemdUtils.types.services;
      };

      sockets = mkOption {
        default = {};
        type = systemdUtils.types.sockets;
      };

      allUnits = mkOption {
        type = types.package;
      };
    };

    # consumed by nixpkgs' systemd-lib utils
    # probably make private?
    systemd = {
      package = mkOption {
        default = pkgs.systemd;
      };
      globalEnvironment = mkOption {
        default = {};
      };
    };
  };

  config = {
    systemd.user.allUnits = systemdUtils.lib.generateUnits {
      type = "user";
      inherit (cfg) units;
      upstreamUnits = [];
      upstreamWants = [];
      packages = [];
    };

    xdg.configPath."systemd/user" = {
      source = cfg.allUnits;
      recursive = true;
    };

    dag.nodes = lib.mkMerge [
      {
        "systemd-reload" = {
          after = [
            "xdg-config-path-systemd/user"
          ];
          command = [
            "busctl"
            "--user"
            "call"
            "org.freedesktop.systemd1"
            "/org/freedesktop/systemd1"
            "org.freedesktop.systemd1.Manager"
            "Reload"
          ];
        };

        "systemd-generate" = {
          before = [
            "static"
            "xdg-config-path-systemd/user"
          ];
          command = [
            # "${(lib.getExe config.bin.activation-manager)}"
            "activation-manager"
            "--verbose"
            "systemd-generate"
            "--incoming"
            "${config.systemd.user.allUnits}"
          ];
          generatesNodes = true;
        };
      }
    ];

    systemd.user.units =
      mapAttrs' (n: v: nameValuePair "${n}.socket" (socketToUnit n v)) cfg.sockets
      // mapAttrs' (n: v: nameValuePair "${n}.service" (serviceToUnit n v)) cfg.services;
  };
}
