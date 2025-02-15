{ config, pkgs, ... }:
{
  imports = [
    ../nix/home/nixos-module.nix
  ];

  users.users."nixos" = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    packages = [
      (config.activation-manager.mkHome {
        home.file."foo".target = "bar";
      })
    ];
  };

  services.getty.autologinUser = "nixos";
  security.sudo.wheelNeedsPassword = false;
}
