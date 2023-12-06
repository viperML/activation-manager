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
  __functor = _: eval;
  inherit eval;
}
