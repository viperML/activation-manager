{
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib) mkOption mdDoc types;
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
    activation-manager = pkgs.python3.pkgs.callPackage ../package.nix {};
    activate = pkgs.writeShellScriptBin "activate" ''
      ${lib.getExe config.bin.activation-manager} "$@" activate --manifest ${config.manifest}
    '';
    bundle = pkgs.symlinkJoin {
      name = "activation-manager-bundle";
      paths = [
        config.bin.activation-manager
        config.bin.activate
        (pkgs.runCommandLocal "activation-manager-manifest" {} ''
          mkdir -p $out/etc
          ln -vsfT ${config.manifest} $out/etc/activation-manager-manifest.json
        '')
      ];
    };
  };
}
