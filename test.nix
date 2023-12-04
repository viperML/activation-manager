{pkgs ? import <nixpkgs> {}}: let
  am = import ./. pkgs.lib;
in {
  home = am {
    inherit pkgs;
    modules = [
      ./modules/home
      {
        path."test.nix".source = ./test.nix;
      }
    ];
  };
}
