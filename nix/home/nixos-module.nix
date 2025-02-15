{
  pkgs,
  lib,
  ...
}:
let
  inherit (lib) mkOption types;

in
{

  options.activation-manager = {
    mkHome = mkOption {
      type = types.functionTo types.package;
      readOnly = true;
    };
  };

  config = {
    systemd.user.services."activation-manager" = {
      wantedBy = [ "default.target" ];
      script = ''
        activate="/etc/profiles/per-user/$USER/bin/activate"
        if [[ -f "$activate" ]]; then
          exec "$activate"
        else
          echo ":: Activation-manager not installed for this user"
        fi
      '';
    };

    activation-manager.mkHome =
      module:
      (import ./default.nix {
        inherit pkgs lib;
        modules = [ module ];
      }).config.build.bundle;
  };

}
