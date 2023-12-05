{
  buildPythonPackage,
  setuptools-scm,
  lib,
  networkx,
}:
buildPythonPackage {
  pname = "activation-manager";
  version = "v0.0.1";
  pyproject = true;
  src = lib.cleanSource ./.;
  nativeBuildInputs = [
    setuptools-scm
  ];
  propagatedBuildInputs = [
    networkx
  ];
  strictDeps = true;
}
