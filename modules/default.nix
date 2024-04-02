{
  config,
  lib,
  pkgs,
  ...
}:
{
  _module.args = {
    # Load nixpkgs' utils
    utils = import "${pkgs.path}/nixos/lib/utils.nix" { inherit lib config pkgs; };
  };

  imports = [
    ./core

    # home specific functionality
    ./home
  ];
}
