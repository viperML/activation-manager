package main;
use v5.38;
use strict;
use warnings;

use ExtUtils::MakeMaker;

WriteMakefile(
    NAME => 'activation-manager',
    VERSION_FROM        => 'lib/AM.pm',
    EXE_FILES => ['bin/activation-manager'],
);