{
  rustPlatform,
  lib,
  pkg-config,
  lua54Packages,
}:
rustPlatform.buildRustPackage {
  name = "activation-manager";

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ../.)) (
      lib.fileset.unions [
        ../src
        ../Cargo.toml
        ../Cargo.lock
      ]
    );
  };

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    lua54Packages.lua
  ];
}
