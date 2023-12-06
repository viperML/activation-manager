{
  lib,
  mkPathConfig,
  mkPathOption,
  config,
  ...
}: {
  options = {
    xdg = {
      configPath = mkPathOption ".config/";
    };
  };

  config = lib.mkMerge [
    (mkPathConfig config.xdg.configPath "xdg-config-path")
    {
      flavor = "home";
      root.location.command = [
        "printenv"
        "HOME"
      ];
      static.location.command = [
        "sh"
        "-c"
        "echo \"$AM_ROOT/.config/static\""
      ];
    }
  ];
}
