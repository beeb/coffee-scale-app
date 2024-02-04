# This example demonstrates a simple temperature sensor peripheral.
#
# The sensor's local value updates every second, and it will notify
# any connected central every 10 seconds.

import struct

import bluetooth
from ble_advertising import advertising_payload
from micropython import const

_IRQ_CENTRAL_CONNECT = const(1 << 0)
_IRQ_CENTRAL_DISCONNECT = const(1 << 1)

# org.bluetooth.service.automation_io
_AUTOMATION_IO_UUID = bluetooth.UUID(0x1815)
# org.bluetooth.characteristic.analog_output
_CHAR_WEIGHT_ANALOG = (bluetooth.UUID(0x2A59), bluetooth.FLAG_READ | bluetooth.FLAG_NOTIFY)

_AUTOMATION_IO_SERVICE = (_AUTOMATION_IO_UUID, (_CHAR_WEIGHT_ANALOG,))

# org.bluetooth.service.battery_service
_BATTERY_UUID = bluetooth.UUID(0x180F)
# org.bluetooth.characteristic.battery_level
_CHAR_BATTERY_LEVEL = (bluetooth.UUID(0x2A19), bluetooth.FLAG_READ)

_BATTERY_SERVICE = (_BATTERY_UUID, (_CHAR_BATTERY_LEVEL,))

# org.bluetooth.characteristic.gap.appearance.xml
_ADV_APPEARANCE_GENERIC_WEIGHT_SCALE = const(3200)


class BLEScales:
    def __init__(self, ble, name="mpy-coffee"):
        self._ble = ble
        self._ble.active(True)
        print('bt activated')
        self._ble.irq(self._irq)
        ((self._weight_handle,), (self._battery_handle,),) = self._ble.gatts_register_services(
            (_AUTOMATION_IO_SERVICE, _BATTERY_SERVICE)
        )
        self._connections = set()
        self._payload = advertising_payload(
            name=name, services=[_AUTOMATION_IO_UUID, _BATTERY_UUID], appearance=_ADV_APPEARANCE_GENERIC_WEIGHT_SCALE,
        )
        self._advertise()

    def _irq(self, event, data):
        # Track connections so we can send notifications.
        if event == _IRQ_CENTRAL_CONNECT:
            conn_handle, _, _, = data
            self._connections.add(conn_handle)
        elif event == _IRQ_CENTRAL_DISCONNECT:
            conn_handle, _, _, = data
            self._connections.remove(conn_handle)
            # Start advertising again to allow a new connection.
            self._advertise()

    def set_weight(self, weight, notify=False):
        # Data is sint16 in hundreth of a gram, signed.
        # Write the local value, ready for a central to read.
        self._ble.gatts_write(self._weight_handle, struct.pack("!h", int(weight * 100)))
        if notify:
            for conn_handle in self._connections:
                # Notify connected centrals to issue a read.
                self._ble.gatts_notify(conn_handle, self._weight_handle)

    def set_battery_level(self, battery):
        self._ble.gatts_write(self._battery_handle, struct.pack("!B", int(battery)))

    def _advertise(self, interval_us=500000):
        self._ble.gap_advertise(interval_us, adv_data=self._payload)
