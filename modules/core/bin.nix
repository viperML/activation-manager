{
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib) mkOption types;
in {
  options.bin = {
    activation-manager = mkOption {
      type = types.package;
    };

    activate = mkOption {
      type = types.package;
    };

    bundle = mkOption {
      type = types.package;
    };
  };

  config.bin = {
    activation-manager = pkgs.callPackage ../../package.nix {};
    activate = pkgs.writeShellScriptBin "activate" ''
      exec ${lib.getExe config.bin.activation-manager} activate "$@" ${config.manifest}/manifest.rn
    '';
    bundle = pkgs.symlinkJoin {
      name = "activation-manager-bundle";
      paths = [
        config.bin.activation-manager
        config.bin.activate
        (pkgs.runCommandLocal "activation-manager-manifest" {} ''
          mkdir -p $out
          ln -vsfT ${config.manifest} $out/activation-manager-manifest.rn
        '')
      ];
    };
  };
}
