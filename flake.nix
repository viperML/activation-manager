{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ] (system: function nixpkgs.legacyPackages.${system});
  in
    (import ./. nixpkgs.lib)
    // {
      packages = forAllSystems (pkgs: {
        default = pkgs.python3.pkgs.callPackage ./package.nix {};
        example-home = with pkgs;
          buildEnv {
            name = "home";
            paths = [
              # regular packages
              pkgs.eza
              pkgs.neofetch

              (self.lib.home-bundle {
                inherit pkgs;
                modules = [
                  {
                    xdg.configPath."hosts".source = "/etc/hosts";
                    xdg.configPath."someDir".source = "/tmp";
                  }
                ];
              })
            ];
          };
      });

      devShells = forAllSystems (pkgs: {
        default = with pkgs;
          mkShellNoCC {
            packages = [
              (python3.withPackages (pp: [
                pp.networkx
              ]))
            ];
          };
      });
    };
}
