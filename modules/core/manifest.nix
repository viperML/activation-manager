{
  lib,
  pkgs,
  config,
  ...
}: let
  inherit (lib) mkOption types;
in {
  options = {
    manifest = mkOption {
      type = types.package;
      description = "Activation-manager manifest.";
      readOnly = true;
    };
  };

  config = {
    manifest = let
      node2rune = {
        name,
        value,
      }: ''
        #{
          name: "${name}",
          after: ${builtins.toJSON value.after},
          before: ${builtins.toJSON value.before},
          ${lib.optionalString (value.action != null) ''
          action: || {
            ${value.action}
          }''}
        },
      '';
    in
      pkgs.writeTextFile {
        name = "activation-manager-manifest";
        text = ''
          pub fn mk_nodes() {[
            ${lib.pipe config.nodes [lib.attrsToList (map node2rune) (lib.concatStringsSep "\n")]}
          ]}
        '';
        destination = "/manifest.rn";
      };
  };
}
