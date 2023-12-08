from typing import NamedTuple
from dasbus.connection import SessionMessageBus


# https://www.freedesktop.org/wiki/Software/systemd/dbus
# job modes:
# https://github.com/systemd/systemd/blob/59b13e07f229a7b8d1ef1565b64c4f019928443a/src/core/job.h#L73-L85

class Unit(NamedTuple):
    name: str
    description: str
    load_state: str
    active_state: str
    sub_state: str
    unit_follow: str
    object_path: str
    job: int
    job_type: str
    job_object_path: str

def cleanable_unit(unit: Unit) -> bool:
    return (unit.load_state == "not-found") and (unit.active_state == "active")



if __name__ == '__main__':
    print("Hello, world!")
    bus = SessionMessageBus()

    systemd = bus.get_proxy(
        service_name="org.freedesktop.systemd1",
        object_path="/org/freedesktop/systemd1",
        interface_name="org.freedesktop.systemd1.Manager"
    )

    res = systemd.Reload()


    not_found = [Unit(*unit) for unit in systemd.ListUnitsFiltered((
        ["not-found"]
    ))]

    cleanable = [
        unit for unit in not_found
        if cleanable_unit(unit)
    ]

    for unit in cleanable:
        systemd.StopUnit(unit.name, "replace")

    pass