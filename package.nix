{ rustPlatform, lib, pkg-config, lua5_4 }:
let
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
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    lua5_4
  ];
  env = {
    LUA_LINK = "dylib";
  };
  strictDeps = true;
  meta.mainProgram = "activation-manager";
}
