import argparse
import sys
import json
from pathlib import Path
from os import environ
import subprocess
from typing import Any, Dict, List
import networkx as nx
import logging
from logging import info, debug

def main() -> int:
    parser = argparse.ArgumentParser(
        prog="activation-manager",
        description="Activation manager CLI",
        epilog="Reference implementation of the manifest.json activator"
    )
    parser.add_argument("-m", "--manifest", required=True, type=Path)
    parser.add_argument("-v", "--verbose", action="store_true")
    args = parser.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(levelname)s %(message)s",
    )

    info("Hello world")


    manifest_file: Path = args.manifest

    with open(manifest_file, "r") as f:
        manifest = json.load(f)


    for (node, env_var) in [("root", "AM_ROOT"), ("static", "AM_STATIC")]:
        cmd: List[str]
        abs: str
        debug(f":: {env_var}")
        if cmd := manifest[node]["location"]["command"]:
            debug(f"{cmd=}")
            res = subprocess.run(cmd , capture_output=True)

            debug(f"{res.stdout=}")
            debug(f"{res.stderr=}")

            if res.returncode != 0:
                return res.returncode

            environ[env_var] = res.stdout.decode("utf-8").strip()

        elif abs := manifest[node]["location"]["absolute"]:
            environ[env_var] = abs

        debug(f"{environ[env_var]=}")

    cmd = [
        "nix",
        "build",
        manifest["static"]["result"],
        "--out-link",
        environ["AM_STATIC"],
    ]
    subprocess.run(cmd, check=True)

    G = nx.DiGraph()

    for (node_name, node_value) in manifest["dag"]["nodes"].items():
        G.add_node(node_name, **node_value)

    for (node_name, node_value) in manifest["dag"]["nodes"].items():
        after: List[str] = node_value["after"]

        for a in after:
            G.add_edge(a, node_name)


    for node_name in nx.topological_sort(G):
        # check for None
        info(f"Running activation for {node_name}")
        if cmd := G.nodes[node_name]["command"]:
            info(f"{cmd}")
            subprocess.run(cmd, check=True)

    return(0)

if __name__ == "__main__":
    sys.exit(main())