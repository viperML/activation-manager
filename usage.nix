let
  pkgs = import <nixpkgs> {};
  activation-manager = import ./. pkgs.lib;
in
  pkgs.buildEnv {
    name = "home";
    paths = [
      pkgs.neofetch
      (activation-manager.lib.home-bundle {
        inherit pkgs;
        modules = [
          {
            xdg.configPath."hosts".source = "/etc/hosts";
            xdg.configPath."someDir".source = "/tmp";
          }
        ];
      })
    ];
  }
