{ config, pkgs, ... }:
let
  mkHome =
    mod:
    (import ../nix/home/default.nix {
      modules = [ mod ];
    }).config.build.bundle;

in
{
  imports = [
    ../nix/home/nixos-module.nix
  ];

  users.users."nixos" = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    packages = [
      (mkHome {
        home.file."foo".target = "bar";
      })
    ];
  };

  services.getty.autologinUser = "nixos";
  security.sudo.wheelNeedsPassword = false;
}
