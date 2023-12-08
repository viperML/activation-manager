{pkgs ? import <nixpkgs> {}}: let
  am = import ./. pkgs.lib;
in
  am.lib {
    inherit pkgs;
    modules = [
      ./modules/home
      {
        xdg.configPath."test3.nix".source = "/etc/hosts";

      }
    ];
  }
