lib: let
  eval = {
    pkgs,
    modules ? [],
    specialArgs ? {},
  }:
    lib.evalModules {
      modules = [./modules] ++ modules;
      specialArgs =
        {
          inherit pkgs;
        }
        // specialArgs;
    };
in {
  __functor = _: eval;
  inherit eval;
}
