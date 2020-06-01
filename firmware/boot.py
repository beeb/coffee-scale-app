# This file is executed on every boot (including wake-boot from deepsleep)
# import esp
# esp.osdebug(None)
import machine
import network

# import webrepl


def do_connect():
    sta_if = network.WLAN(network.STA_IF)
    if not sta_if.isconnected():
        print('Connecting to network...')
        sta_if.active(True)
        sta_if.connect('Potato', 'moriasweetmoria')
        while not sta_if.isconnected():
            pass
    print('Network config:', sta_if.ifconfig())


machine.freq(80000000)
# webrepl.start()
# do_connect()
