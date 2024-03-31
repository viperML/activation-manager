{ amUtils, config, ... }:
{
  options.path = amUtils.mkPathOption "";

  config = amUtils.mkPathConfig config.path "path";
}
