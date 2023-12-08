from typing import NamedTuple
from dasbus.connection import SessionMessageBus, InterfaceProxy
import configparser
from configparser import ConfigParser
from pathlib import Path
from os import environ

# https://www.freedesktop.org/wiki/Software/systemd/dbus
# job modes:
# https://github.com/systemd/systemd/blob/59b13e07f229a7b8d1ef1565b64c4f019928443a/src/core/job.h#L73-L85

def parse_ini(path: Path) -> ConfigParser:
    config = configparser.ConfigParser()
    config.read(path)
    return config


def unit_for(systemd_proxy: InterfaceProxy, name: str) -> InterfaceProxy:
    path = systemd_proxy.GetUnit(name)
    return bus.get_proxy(
        service_name="org.freedesktop.systemd1",
        object_path=path,
        interface_name="org.freedesktop.systemd1.Unit"
    )



if __name__ == '__main__':
    bus = SessionMessageBus()

    systemd_proxy = bus.get_proxy(
        service_name="org.freedesktop.systemd1",
        object_path="/org/freedesktop/systemd1",
        interface_name="org.freedesktop.systemd1.Manager"
    )

    print(":: Reloading system")
    res = systemd_proxy.Reload()

    for (name, *rest) in systemd_proxy.ListUnitsFiltered((["active"])):
        unit = unit_for(systemd_proxy, name)

        if unit.ActiveState != "active":
            break

        for wants in unit.Wants:
            child_unit = unit_for(systemd_proxy, wants)

            if child_unit.ActiveState != "active":
                print("Child not active")
                print(f"{unit.Id=} {child_unit.Id=}")
                print(f"{child_unit.Transient=}")

            pass


        pass




    pass