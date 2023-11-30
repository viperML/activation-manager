{pkgs ? import <nixpkgs> {}}: let
  mod = {config, ...}: {
    rootPath.absolute = "/tmp";
    path."test.nix".source = ./test.nix;
    path."hosts".source = "/etc/hosts";
  };

  activation-manager = (import ./default.nix pkgs.lib) {
    inherit pkgs;
    modules = [mod];
  };
in {
  evalResult = activation-manager;
}
