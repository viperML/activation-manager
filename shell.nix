with import <nixpkgs> { };
pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc

    pkgs.rust-analyzer
    pkgs.rustfmt

    # If the dependencies need system libs, you usually need pkg-config + the lib
    pkgs.pkg-config
    lua54Packages.lua
    lua-language-server
  ];

  env = {
    RUST_BACKTRACE = "full";
    RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  };
}
