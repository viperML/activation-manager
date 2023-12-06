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
      # TODO: start units
      {
        "systemd-user-reload" = {
          after = ["xdg-config-path-systemd/user"];
          command = [
            "systemctl"
            "--user"
            "daemon-reload"
          ];
        };

        "systemd-user-stop-not-found" = {
          after = ["systemd-user-reload"];
          command = [
            (lib.getExe (pkgs.writeShellScriptBin "systemd-stop-lingering" ''
              systemctl --user list-units --state=not-found --plain --no-legend \
              | ${lib.getExe pkgs.gnugrep} -v inactive \
              | sed 's/\s\+.*//' \
              | while IFS= read -r line; do
                systemctl --user stop $line
              done
            ''))
          ];
        };
      }
    ];

    systemd.user.units =
      mapAttrs' (n: v: nameValuePair "${n}.socket" (socketToUnit n v)) cfg.sockets
      // mapAttrs' (n: v: nameValuePair "${n}.service" (serviceToUnit n v)) cfg.services;
  };
}
