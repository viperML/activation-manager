lib: let
  eval = {
    pkgs,
    modules ? [],
    # specialArgs ? {},
  }:
    lib.evalModules {
      modules = [./modules] ++ modules;
      # specialArgs =
      #   {
      #     inherit pkgs;
      #   }
      #   // specialArgs;
      specialArgs =
        {
          inherit pkgs;
        }
        // (import ./modules/.mkPath.nix {inherit pkgs lib;});
    };
in {
  lib = {
    __functor = _: eval;
    inherit eval;
    home-bundle = args:
      (eval (args
        // {
          modules =
            args.modules
            ++ [
              ./modules/home
            ];
        }))
      .config
      .bin
      .bundle;
  };
}
