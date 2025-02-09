{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
  modules ? [ ],
  ...
}:
lib.evalModules {
  modules = [ ./core.nix ] ++ modules;
  specialArgs = { inherit pkgs; };
}
