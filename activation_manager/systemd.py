from argparse import Namespace
from dataclasses import dataclass, field
from typing import NamedTuple
from dasbus.connection import SessionMessageBus, InterfaceProxy
import configparser
from configparser import ConfigParser
from pathlib import Path
from os import environ
from enum import Enum
from typing import List, Dict
import os
from logging import info, debug
import json
import sys
from dasbus.error import DBusError

# https://www.freedesktop.org/wiki/Software/systemd/dbus
# job modes:
# https://github.com/systemd/systemd/blob/59b13e07f229a7b8d1ef1565b64c4f019928443a/src/core/job.h#L73-L85


class Action(Enum):
    START = "start"
    RESTART = "restart"
    DESTROY = "destroy"

# TODO: Maybe move to codegen from jsonschema
@dataclass
class Node():
    command: List[str]
    after: List[str] = field(default_factory=list)
    before: List[str] = field(default_factory=list)
    generatesNodes: bool = False

def queue_unit(nodes: Dict[str, Node], unit: str, action: Action):
    info(f"Queueing {unit} for {action}")
    nodes[unit] = Node(
        command = [
            "activation-manager",
            "systemd-handle-unit",
            "--unit",
            unit,
            "--action",
            action.value
        ]
    )

def get_current_root() -> Path:
    # FIXME
    current_root = Path(
        environ["AM_ROOT"],
        ".config",
        "systemd",
        "user"
    )
    debug(f"{current_root=}")
    return current_root

def generate(args: Namespace) -> int:
    res = dict()
    incoming_root = args.incoming
    debug(f"{incoming_root=}")

    current_root = get_current_root()

    common_units = set()

    for p in incoming_root.iterdir():
        if not p.name.endswith(".wants"):
            common_units.add(p.name)

    for p in current_root.iterdir():
        if not p.name.endswith(".wants"):
            common_units.add(p.name)

    info(f"Common units: {common_units}")

    for u in common_units:
        current_exists = (current_root / u).is_symlink()

        current_dead =  not (current_root / u).exists() and current_exists
        if current_dead:
            queue_unit(res, u, Action.DESTROY)
            continue

        incoming_exists = (incoming_root / u).exists()

        if incoming_exists and not current_exists:
            queue_unit(res, u, Action.START)
            continue

        if not incoming_exists and current_exists:
            queue_unit(res, u, Action.DESTROY)
            continue

        if incoming_exists and current_exists:
            with open(incoming_root / u) as f:
                incoming_content = f.read()

            with open(current_root / u) as g:
                current_content = g.read()

            if incoming_content != current_content:
                queue_unit(res, u, Action.RESTART)
                continue
            else:
                info(f"Skipping {u}, no changes")
                debug(f"{incoming_content=}")
                debug(f"{current_content=}")
                continue


    ser = json.dumps(res, default=lambda o: o.__dict__, indent=2)
    print(ser)


def handle_unit(args: Namespace) -> int:
    unit = args.unit
    action = Action(args.action)

    info(f"Handling {unit} with {action}")
    current_root = get_current_root()

    current_unit_path: Path = current_root / args.unit
    debug(f"{current_unit_path=}")
    if action == Action.DESTROY and current_unit_path.is_symlink():
        info("Removing unit file")
        os.unlink(current_unit_path)

    bus = SessionMessageBus()
    systemd_proxy = bus.get_proxy(
        service_name="org.freedesktop.systemd1",
        object_path="/org/freedesktop/systemd1",
        interface_name="org.freedesktop.systemd1.Manager"
    )

    try:
        res = systemd_proxy.GetUnit(args.unit)
    except DBusError:
        info(f"Unit {unit} not found")
        return 0

    unit_proxy = bus.get_proxy(
        service_name="org.freedesktop.systemd1",
        object_path=res,
        interface_name="org.freedesktop.systemd1.Unit"
    )

    match action:
        case Action.DESTROY:
            if unit_proxy.LoadState == "loaded":
                info("Stopping")
                unit_proxy.Stop("replace")
            else:
                info("Not loaded, skipping")

        case Action.START:
            info("Starting")
            unit_proxy.Start("replace")

        case Action.RESTART:
            info("Restarting")
            unit_proxy.Restart("replace")

    return 0