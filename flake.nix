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
      });
    };
}
