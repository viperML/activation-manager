{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: function nixpkgs.legacyPackages.${system});
    in
    {
      lib = {
        home = import ./nix/home/default.nix;
        homeBundle = args: (self.lib.home args).config.build.bundle;
      };

      nixosModules.home = ./nix/home/nixos-module.nix;

      packages = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/package.nix { };
        optionsJSON =
          (pkgs.nixosOptionsDoc {
            options =
              (self.lib.home {
                inherit pkgs;
              }).options;
          }).optionsJSON;
      });

      devShells = forAllSystems (pkgs: {
        default =
          with pkgs;
          mkShell {
            packages = [
              pkgs.cargo
              pkgs.rustc
              pkgs.clippy

              pkgs.rust-analyzer
              pkgs.rustfmt
              pkgs.yaml-language-server

              # If the dependencies need system libs, you usually need pkg-config + the lib
              pkgs.pkg-config
              lua54Packages.lua
              lua-language-server

              (pkgs.nodejs.override { enableNpm = false; })
              pkgs.pnpm
            ];
            env = {
              RUST_BACKTRACE = "full";
              RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
              OPTIONS_JSON = self.packages.${system}.optionsJSON;
            };
          };
      });
    };
}
