{
  amUtils,
  config,
  lib,
  pkgs,
  ...
}:
{
  options = {
    home.paths = amUtils.mkPathOption "";
  };

  config = {
    static.location = lib.singleton (
      pkgs.writeShellScript "get-static" ''
        set -ex
        if [[ -z "''${XDG_CONFIG_HOME}" ]]; then
          echo "$HOME/.config/activation-manager-static"
        else
          echo "$XDG_CONFIG_HOME/activation-manager-static"
        fi
      ''
    );

    home.paths = amUtils.mkPathConfig config.path.home "path-home";
  };
}
