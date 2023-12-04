package AM::Command::switch;
use v5.38;
use strict;
use warnings;
use AM -command;

use Data::Dumper;
use JSON;
use File::Slurp;
use String::Util qw(trim);

sub abstract { "Run activation-manager" }

sub opt_spec {
    return (
        [ "flakeref|F=s", "flake output" ],
        [ "manifest|m=s", "manifest file" ]
    );
}

sub execute {
    my ( $self, $opt, $args ) = @_;

    say $opt;
    say Dumper($opt);

    my $outpath;

    if ( defined $opt->{flakeref} ) {
        my $flakref  = $opt->{flakeref};
        my $manifest = "$flakref.config.manifest^out";
        say $manifest;

        $outpath = `nix build $manifest --no-link --print-out-paths`;
        $outpath = trim($outpath);
    }
    elsif ( defined $opt->{manifest} ) {
        $outpath = trim($opt->{manifest});
    }
    else {
        die "No flake or manifest specified";
    }
    say $outpath;

    # read outpath as JSON

    my $contents = read_file($outpath);
    my $json     = decode_json($contents);

    say Dumper($json);

    if ( defined $json->{root}->{location}->{absolute} ) {
        $ENV{AM_ROOT} = $json->{root}->{location}->{absolute};
    }
    else {
        my $cmd = $json->{root}->{location}->{command};
        my $res = `$cmd`;
        $ENV{AM_ROOT} = trim($res);
    }

    if ( defined $json->{static}->{location}->{absolute} ) {
        $ENV{AM_STATIC} = $json->{static}->{location}->{absolute};
    }
    else {
        my $cmd = $json->{static}->{location}->{command};
        say $cmd;
        my $res = `$cmd`;
        $ENV{AM_STATIC} = trim($res);
    }

    system("printenv");

    my $static_result = $json->{static}->{result};
    `ln -vsfT $static_result $ENV{AM_STATIC}`;

    while ( my ( $key, $value ) = each %{ $json->{dag}->{nodes} } ) {
        say "Running activation for $key";
        system $value->{exec};
    }
}

1;
