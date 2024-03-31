{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib) mkOption mdDoc types;
in
{
  _module.args = {
    # Load nixpkgs' utils
    utils = import "${pkgs.path}/nixos/lib/utils.nix" { inherit lib config pkgs; };
  };

  imports = [
    ./core/root.nix
    ./core/nodes.nix
    ./core/static.nix
    ./core/manifest.nix
    ./core/bin.nix

    # Functionality
    ./path.nix
  ];
}
