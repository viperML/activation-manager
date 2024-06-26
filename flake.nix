{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs
      [
        "x86_64-linux"
        "aarch64-linux"
      ]
      (system: function nixpkgs.legacyPackages.${system});
  in
    (import ./. nixpkgs.lib)
    // {
      packages = forAllSystems (
        pkgs: rec {
          default = pkgs.callPackage ./package.nix {};
          dev = default.overrideAttrs (_: {src = null;});
          example-home = let
            eval = self.lib {
              inherit pkgs;
              modules = [
                ./modules
                # ./modules/home
                # ./test/home.nix
                ./test/core.nix
              ];
            };
          in
            pkgs.symlinkJoin {
              name = "home";
              paths = [
                # pkgs.eza
                eval.config.bin.bundle
              ];
              # inherit eval;
              passthru = {
                inherit eval;
              };
            };
        }
      );

      devShells = forAllSystems (
        pkgs: {
          shell = with pkgs;
            mkShell {
              packages = [
                cargo
                rustc
                rustfmt
                rust-analyzer-unwrapped
                rune
              ];
              env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
            };
        }
      );
    };
}
