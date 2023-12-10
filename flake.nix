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
      packages = forAllSystems (pkgs: rec {
        default = pkgs.python3.pkgs.callPackage ./package.nix {};
        dev = default.overrideAttrs (_: {src = null;});
        example-home = let
          eval = self.lib {
            inherit pkgs;
            modules = [
              ./modules/home
              ./test/home.nix
            ];
          };
        in
          with pkgs;
            (buildEnv {
              name = "home";
              paths = [
                # regular packages
                pkgs.eza
                pkgs.neofetch

                eval.config.bin.bundle
              ];
            })
            .overrideAttrs (_: {
              passthru = {inherit eval;};
            });
      });

      devShells = forAllSystems (pkgs: {
        shell = with pkgs;
          mkShellNoCC {
            packages = [
            ];
          };
      });
    };
}
