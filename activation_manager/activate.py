from argparse import Namespace
from ast import Dict
import json
from pathlib import Path
from os import environ
import subprocess
from typing import Any, List
import networkx as nx
from logging import info, debug, error
from ansi.colour import fg
import io
import sys
from . import escape_ansi

def fill_graph(G: nx.DiGraph, nodes: dict[str, Any]):
    for (node_name, node_value) in nodes.items():
        G.add_node(node_name, **node_value)

    for (node_name, node_value) in nodes.items():
        after: List[str] = node_value["after"]
        before: List[str] = node_value["before"]

        for a in after:
            G.add_edge(a, node_name)

        for b in before:
            G.add_edge(node_name, b)

def main(args: Namespace) -> int:
    info("Welcome to a activation-manager")

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

        else:
            error(f"Neither command nor absolute path specified for {env_var}")
            return 1

        debug(f"{environ[env_var]=}")


    G_next = nx.DiGraph()
    fill_graph(G_next, manifest["dag"]["nodes"])

    while G_next.number_of_nodes() > 0:
        G = G_next.copy()
        G_next = nx.DiGraph()
        pass

        for node_name in nx.topological_sort(G):
            # check for None
            info(f"Activating: {fg.bold}{node_name}")
            try:
                cmd = G.nodes[node_name]["command"]
            except KeyError:
                info(f"{node_name} has no command")
                continue

            cmd_human = " ".join(cmd)
            print(f"{fg.brightblack}{node_name}> {cmd_human}", file=sys.stderr)

            if not G.nodes[node_name]["generatesNodes"]:
                proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
                for line in io.TextIOWrapper(proc.stdout, encoding="utf-8"):
                    l = escape_ansi(line.strip())
                    print(f"{fg.brightblack}{node_name}> {l}", file=sys.stderr)

                if proc.wait() != 0:
                    error(f"{node_name} failed")
                    return 1

            else:
                res = subprocess.run(
                    cmd,
                    capture_output=True,
                    text=True
                )

                # print(res.stdout)
                # print(res.stderr)
                new_nodes = json.loads(res.stdout)
                debug(f"new nodes: {new_nodes}")
                fill_graph(G_next, new_nodes)
                pass






    return 0