# activation-manager

Activation-manager is the next-gen alternative to home-manager, nix-darwin
or system-manager, based on a framework for dynamic activation of tasks.

Each task is programmed through a Nix + Lua interface, which is ran by a Rust
backend upon activation. Tasks may vary from symlinking files, to running other
programs.

## Installation and usage

Actiation-manager uses an activation bundle, which is a regular package that you
can build and run. Unlike home-manager, activation-manager doesn't require
a custom CLI that points into a configuration, because it's a package that you 
run and build for yourself.

For example, a home activation bundle in a flake:

~~~nix
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.activation-manager.url = "github:viperML/activation-manager";
  outputs = {self, nixpkgs, activation-manager}: let 
    system = "x86_64-linux"; 
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    # 1. Export a new package as an AM bundle
    packages.${system}.default = activation-manager.lib.homeBundle {
      inherit pkgs;
      modules = [{
        # 2, Declare your AM configuration
        home.file."foo".target = "bar";
      }];
    };
  };
}

# 3. Run Activation-Manager
# nix run .
~~~

To use Activation-Manager as a NixOS module:


~~~nix
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.activation-manager.url = "github:viperML/activation-manager";
  outputs = {self, nixpkgs, activation-manager}: let 
    system = "x86_64-linux"; 
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
      modules = [
        # 1. Add AM to your imports
        activation-manager.nixosModules.home

        ({config, pkgs, ...}: {
          users.users."nixos" = {
            packages = [
              # 2. Add an AM bundle to one of your user's packages
              (config.activation-manager.mkBundle {
                home.file."foo".target = "bar";
              })
            ];
          };
        })
      ];
    };
  };
}
~~~

## Documentation

TODO


## Features and roadmap

  - [x] Node graph runtime
    - [ ] Run nodes in parallel
    - [ ] Create nodes dynamically
    - [ ] Before, after and id's
    - [ ] Error recovery
    - [ ] Rollback system

  - Node types
    - [x] File symlink node type
      - [ ] Copy files
      - [ ] Recursive for folders
    - [x] Exec node type
    - [ ] Lua node type
    - [ ] Systemd unit node type
    - [x] Dconf node type

  - [ ] Nix API for dealing with Lua expressions vs strings
  - [ ] Lua type hints
  - [ ] Documentation site

  - [x] Home-manager-like Nix library
    - [x] Use static directory
    - [x] Installable from NixOS

