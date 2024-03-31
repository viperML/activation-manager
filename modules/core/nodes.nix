{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib) mkOption types;
in
{
  options.nodes = mkOption {
    description =  "Activation nodes";
    default = { };
    type = types.attrsOf (
      types.submodule (
        { config, name, ... }:
        {
          options = {
            after = mkOption {
              type = types.listOf types.str;
              default = [ ];
              description = ''
                List of nodes that must finish before this.
              '';
            };

            before = mkOption {
              type = types.listOf types.str;
              default = [ ];
              description = ''
                List of nodes that must wait for this to finish.
              '';
            };

            command = mkOption {
              type = types.nullOr (types.listOf (types.either types.str types.package));
              default = null;
              description = ''
                Command to execute. Note that this is passed directly to a system call,
                so no variable interpolation from bash is allowed here.
              '';
            };

            generatesNodes = mkOption {
              type = types.bool;
              default = false;
              description = "This node generates more nodes dynamically from stdout";
            };
          };
        }
      )
    );
  };
}
