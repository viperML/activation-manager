{
  buildPythonPackage,
  setuptools-scm,
  lib,
  networkx,
  dasbus,
  makeWrapper,
  ansi,
  colorlog,
}:
buildPythonPackage {
  pname = "activation-manager";
  version = "v0.0.1";
  pyproject = true;
  src = lib.fileset.toSource {
    root = ./.;
    fileset =
      lib.fileset.intersection
      (lib.fileset.fromSource (lib.sources.cleanSource ./.))
      (lib.fileset.unions [
        ./activation_manager
        ./pyproject.toml
      ]);
  };
  nativeBuildInputs = [
    setuptools-scm
    makeWrapper
  ];
  propagatedBuildInputs = [
    networkx
    dasbus
    ansi
    colorlog
  ];
  postFixup = ''
    wrapProgram $out/bin/activation-manager --prefix PATH : "$out/bin"
  '';
  strictDeps = true;
  meta.mainProgram = "activation-manager";
}
