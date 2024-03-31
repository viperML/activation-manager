lib:
let
  eval =
    {
      pkgs,
      modules ? [ ],
    # specialArgs ? {},
    }:
    lib.evalModules {
      modules = [ ./modules ] ++ modules;
      # specialArgs =
      #   {
      #     inherit pkgs;
      #   }
      #   // specialArgs;
      specialArgs = {
        inherit pkgs;
        amUtils = { } // (import ./utils/mkPath.nix { inherit pkgs lib; });
      };
    };
in
{
  lib = {
    __functor = _: eval;
    inherit eval;
    home-bundle =
      args: (eval (args // { modules = args.modules ++ [ ./modules/home ]; })).config.bin.bundle;
  };
}
