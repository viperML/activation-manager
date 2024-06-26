{
  rustPlatform,
  lib,
}: let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
in
  rustPlatform.buildRustPackage {
    pname = "activation-manager";
    inherit (cargoToml) version;
    cargoLock.lockFile = ./Cargo.lock;
    src = lib.fileset.toSource {
      root = ./.;
      fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./.)) (
        lib.fileset.unions [
          ./src
          ./Cargo.toml
          ./Cargo.lock
        ]
      );
    };
    strictDeps = true;
    meta.mainProgram = "activation-manager";
  }
