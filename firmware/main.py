"""Main file running on the scales ESP32."""
import time

import bluetooth
from ble_scales import BLEScales

# from filtering import KalmanFilter


def main():
    ble = bluetooth.BLE()
    scales = BLEScales(ble)
    # kf = KalmanFilter(0.2, 0.5)

    scales.set_battery_level(86)

    start = time.ticks_ms()
    while True:
        time_delta = time.ticks_diff(time.ticks_ms(), start)
        if time_delta < 10000:
            scales.set_weight(0, notify=True)
        elif time_delta < 35000:
            scales.set_weight((time_delta - 10000) * 1.6 / 1000, notify=True)
        elif time_delta < 45000:
            scales.set_weight(40, notify=True)
        else:
            start = time.ticks_ms()

        # filtered_weight = kf.update_estimate(filtered_weight)
        # scales.set_weight(filtered_weight, notify=True)
        time.sleep_ms(250)


if __name__ == "__main__":
    main()
