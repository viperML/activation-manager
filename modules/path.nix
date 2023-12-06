{
  mkPathConfig,
  mkPathOption,
  config,
  lib,
  ...
}: {
  options = {
    path = mkPathOption "";
  };

  config = lib.mkMerge [
    (mkPathConfig config.path "path")
    {
      _module.args = {inherit mkPathOption mkPathConfig;};
    }
  ];
}
