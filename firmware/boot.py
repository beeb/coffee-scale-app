import network


def do_connect():
    sta_if = network.WLAN(network.STA_IF)
    if not sta_if.isconnected():
        print('Connecting to network...')
        sta_if.active(True)
        sta_if.connect('SSID', 'password')
        while not sta_if.isconnected():
            pass
    print('Network config:', sta_if.ifconfig())


# uncomment to setup WiFi connection
# do_connect()
