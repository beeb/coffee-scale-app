"""Main file running on the scales ESP32."""
import time

import _thread
import bluetooth
import micropython
from art import BATTERY, DOT, GRAM, LOGO, show_digit, show_sprite
from ble_scales import BLEScales
from filtering import KalmanFilter
from hx711 import HX711
from machine import ADC, I2C, Pin
from ssd1306 import SSD1306_I2C

micropython.alloc_emergency_exception_buf(100)

i2c = I2C(-1, scl=Pin(22), sda=Pin(21))
screen = SSD1306_I2C(width=128, height=32, i2c=i2c)
screen.fill(0)
show_sprite(screen, LOGO, 51, 1)
screen.show()

ble = bluetooth.BLE()
print('bt loaded')
scales = BLEScales(ble)
kf = KalmanFilter(0.03, q=0.1)
button_pin = Pin(0, Pin.IN, Pin.PULL_UP)
vsense_pin = ADC(Pin(34))
vsense_pin.atten(ADC.ATTN_11DB)
bat_percent = 0

hx = HX711(dout=14, pd_sck=13, gain=64)
hx.set_scale(1544.667)
hx.tare()
kf.update_estimate(hx.get_units(times=1))
filtered_weight = 0


def tare_callback(pin):
    global hx, kf
    hx.tare(times=3)
    kf.last_estimate = 0.0


def main():
    global filtered_weight, bat_percent, scales, button_pin, hx, kf

    # uncomment next 2 lines to get a load cell reading for calibration (in the console/serial)
    # while True:
    #    print(hx.read_average(times=100))

    battery_sum = 0
    for i in range(10):
        battery_sum += vsense_pin.read()
    bat_percent = adc_to_percent(battery_sum / 10)
    scales.set_battery_level(bat_percent)

    _thread.start_new_thread(display_weight, ())

    button_pin.irq(trigger=Pin.IRQ_FALLING, handler=tare_callback)

    last = 0
    while True:
        weight = hx.get_units(times=1)
        filtered_weight = kf.update_estimate(weight)
        now = time.ticks_ms()
        if time.ticks_diff(now, last) > 100:
            last = now
            rounded_weight = round(filtered_weight / 0.05) * 0.05
            scales.set_weight(rounded_weight, notify=True)


def adc_to_percent(v_adc):
    if v_adc > 2399:  # 4.1-4.2 = 94-100%
        val = int(0.10169492 * v_adc - 149.966)
        return val if val <= 100 else 100
    if v_adc > 2341:  # 4.0-4.1 = 83-94%
        return int(0.18965517 * v_adc - 360.983)
    if v_adc > 2282:  # 3.9-4.0 = 72-83%
        return int(0.18644068 * v_adc - 353.458)
    if v_adc > 2224:  # 3.8-3.9 = 59-72%
        return int(0.22413793 * v_adc - 439.483)
    if v_adc > 2165:  # 3.7-3.8 = 50-59%
        return int(0.15254237 * v_adc - 280.254)
    if v_adc > 2107:  # 3.6-3.7 = 33-50%
        return int(0.29310345 * v_adc - 584.569)
    if v_adc > 2048:  # 3.5-3.6 = 15-33%
        return int(0.30508475 * v_adc - 609.814)
    if v_adc > 1990:  # 3.4-3.5 = 6-15%
        return int(0.15517241 * v_adc - 302.793)
    if v_adc >= 1931:  # 3.3-3.4 = 0-6%
        return int(0.10169492 * v_adc - 196.373)
    return 0


def display_weight():
    global filtered_weight, bat_percent
    while True:
        screen.fill(0)
        rounded_weight = round(filtered_weight / 0.05) * 0.05
        string = '{:.2f}'.format(rounded_weight)
        if len(string) > 6:
            string = '{:.1f}'.format(rounded_weight)
        if string == '-0.00':
            string = '0.00'
        position = 118
        for char in reversed(string):
            if position < 0:
                break
            if char == '-':
                char = 'MINUS'
            if char == '.':
                position -= 7
                if position < 0:
                    break
                show_sprite(screen, DOT, position, 27)
            else:
                position -= 22
                if position < 0:
                    break
                show_digit(screen, char, position, 1)
        show_sprite(screen, GRAM, 117, 16)
        if bat_percent <= 20:
            show_sprite(screen, BATTERY, 117, 1)
        screen.show()


if __name__ == "__main__":
    main()
