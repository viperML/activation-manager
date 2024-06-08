{
  config,
  lib,
  pkgs,
  ...
}: let
  inherit (lib) mkOption types;
in {
  options.nodes = mkOption {
    description = "Activation nodes.";
    default = {};
    type = types.attrsOf (types.submodule ({
      config,
      name,
      ...
    }: {
      options = {
        after = mkOption {
          type = types.listOf types.str;
          default = [];
          description = ''
            List of nodes that must finish before this.
          '';
        };

        before = mkOption {
          type = types.listOf types.str;
          default = [];
          description = ''
            List of nodes that must wait for this to finish.
          '';
        };

        action = mkOption {
          type = types.nullOr types.str;
          default = null;
          description = ''
            Rune code to execute.
          '';
        };
      };
    }));
  };
}
