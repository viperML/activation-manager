# activation-manager

## Usage

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    activation-manager.url = "github:viperML/activation-manager";
  };

  outputs = {self, nixpkgs, activation-manager}: {
    packages."x86_64-linux".default = with nixpkgs.legacyPackages.x86_64-linux;
      buildEnv {
        name = "home";
        paths = [
          # regular packages
          pkgs.eza
          pkgs.neofetch

          (activation-manager.lib.home-bundle {
            inherit pkgs;
            modules = [{
              xdg.configPath."hosts".source = "/etc/hosts";
              xdg.configPath."someDir".source = "/tmp";
            }];
          })
        ];
      };
  };
}
```

```
$ nix profile install /path/to/flake && activate
```