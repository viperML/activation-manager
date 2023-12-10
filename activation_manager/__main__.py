import argparse
import sys
from pathlib import Path
from . import activate, systemd
import logging
from os import environ

def main() -> int:
    toplevel_parser = argparse.ArgumentParser(
        prog="activation-manager",
        description="Activation manager CLI",
    )
    toplevel_parser.add_argument("-v", "--verbose", action="store_true")

    subparsers= toplevel_parser.add_subparsers(
        title="subcommands",
        required=True,
        dest="subcommand",
    )

    activate_parser = subparsers.add_parser(
        "activate",
        help="activate a manifest",
    )
    activate_parser.add_argument("-m", "--manifest", required=True, type=Path)

    systemd_generate_parser = subparsers.add_parser(
        "systemd-generate",
        help="systemd activator for internal use"
    )
    systemd_generate_parser.add_argument("-i", "--incoming", required=True, type=Path)
    systemd_generate_parser.add_argument("-c", "--current", required=False, type=Path, default=Path(
        environ["HOME"],
        ".config",
        "systemd",
        "user"
    ))

    systemd_handle_unit_parser = subparsers.add_parser(
        "systemd-handle-unit",
        help="systemd activator for internal use"
    )
    systemd_handle_unit_parser.add_argument("-u", "--unit", required=True, type=str)
    systemd_handle_unit_parser.add_argument("-a", "--action", required=True, type=str)


    args = toplevel_parser.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(levelname)s %(message)s",
        stream=sys.stderr,
    )

    match args.subcommand:
        case "activate":
            return activate.main(args)

        case "systemd-generate":
            return systemd.generate(args)

        case "systemd-handle-unit":
            return systemd.handle_unit(args)

        case _:
            print("Not implemented")
            return 1

if __name__ == "__main__":
    sys.exit(main())