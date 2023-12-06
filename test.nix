{pkgs ? import <nixpkgs> {}}: let
  am = import ./. pkgs.lib;
in {
  home = am {
    inherit pkgs;
    modules = [
      ./modules/home
      {
        path."test.nix".source = ./test.nix;
        path."test/test2.nix".source = ./test.nix;
        # xdg.configPath."test3.nix".source = "/etc/hosts";
      }
    ];
  };
}
