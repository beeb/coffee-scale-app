"""Main file running on the scales ESP32."""
import time

import bluetooth
from ble_scales import BLEScales
from machine import ADC, Pin

from filtering import KalmanFilter

ble = bluetooth.BLE()
print('bt loaded')
scales = BLEScales(ble)
kf = KalmanFilter(0.2, 0.5)
button_pin = Pin(0, Pin.IN, Pin.PULL_UP)
vsense_pin = ADC(Pin(34))
vsense_pin.atten(ADC.ATTN_11DB)


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


def main():
    print('main reached')
    kf_vsense = KalmanFilter(100, 0.01)
    filtered_adc = kf_vsense.update_estimate(vsense_pin.read())
    for i in range(10):
        filtered_adc = kf_vsense.update_estimate(vsense_pin.read())
        time.sleep_ms(30)
    bat_percent = adc_to_percent(filtered_adc)
    print(bat_percent)
    scales.set_battery_level(bat_percent)

    start = time.ticks_ms()
    while True:
        time_delta = time.ticks_diff(time.ticks_ms(), start)
        print(time_delta)
        if time_delta < 10000:
            scales.set_weight(0, notify=True)
        elif time_delta < 35000:
            scales.set_weight((time_delta - 10000) * 1.6 / 1000, notify=True)
        elif time_delta < 45000:
            scales.set_weight(40, notify=True)
        else:
            start = time.ticks_ms()

        if button_pin.value() == 0:
            start = time.ticks_ms()  # tare

        # filtered_weight = kf.update_estimate(filtered_weight)
        # scales.set_weight(filtered_weight, notify=True)
        time.sleep_ms(250)


if __name__ == "__main__":
    main()
