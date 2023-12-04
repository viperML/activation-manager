let
  pkgs = import <nixpkgs> {};
  pp = pkgs.perlPackages;
in
  pkgs.mkShellNoCC {
    packages = with pp; [
      perl
      PerlCritic
      PadWalker

      AppCmd
      JSON
      FileSlurp
      TermANSIColor
      StringUtil
      DataDumper
    ];
  }
