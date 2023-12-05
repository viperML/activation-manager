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
  in {
    am = import ./test.nix {pkgs = nixpkgs.legacyPackages.x86_64-linux;};

    packages = forAllSystems (pkgs: rec {
      default = pkgs.python3.pkgs.callPackage ./package.nix {};
      env = pkgs.python3.withPackages (_: [default]);
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
