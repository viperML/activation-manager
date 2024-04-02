{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib) mkOption types mdDoc;
in
{
  options = {
    static = {
      derivations = mkOption {
        type = types.attrsOf types.package;
        default = { };
        description = "Derivations to merge into the static package.";
      };

      result = mkOption {
        type = types.package;
        readOnly = true;
        description = "Resulting merge of all static derivations.";
      };

      location = mkOption {
        type = types.listOf (types.either types.str types.package);
        description = "Static directory location, obtained at runtime.";
      };
    };
  };

  config = {
    nodes."static" = {
      command = lib.singleton (pkgs.writeShellScript "static" ''
        nix build ${config.static.result} --out-link "$AM_STATIC"
      '');
    };

    static = {
      result = pkgs.buildEnv {
        name = "am-static";
        paths = builtins.attrValues config.static.derivations;
      };
    };
  };
}
