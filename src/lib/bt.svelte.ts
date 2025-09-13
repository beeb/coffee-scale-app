import { browser } from '$app/environment'

export class Bluetooth {
	private static instance: Bluetooth
	server = $state<BluetoothRemoteGATTServer>()
	enabled = $state(false)
	connected = $state(false)
	newFirmware = $state(true)
	weightCharacteristic = $state<BluetoothRemoteGATTCharacteristic>()
	batteryLevel = $state<number>()
	currentWeight = $state(0)

	private constructor() {
		this.checkStatus()

		if (browser && navigator.bluetooth) {
			navigator.bluetooth.addEventListener('availabilitychanged', () => {
				this.checkStatus()
			})
		}
	}

	private readBatteryLevel = async () => {
		if (!this.server) {
			return
		}
		const service = await this.server.getPrimaryService(0x180F)
		const batteryLevelCharacteristic = await service.getCharacteristic(0x2A19)
		this.batteryLevel = (await batteryLevelCharacteristic.readValue()).getUint8(0)
	}

	private onWeightUpdate = (event: Event) => {
		if (event.target === null) {
			return
		}
		const dataView = (event.target as BluetoothRemoteGATTCharacteristic).value
		// get the value as a int16 or int32 depending on the firmware
		const value = (this.newFirmware ? dataView?.getInt32(0, false) : dataView?.getInt16(0, false)) ?? 0
		this.currentWeight = value / 100.0
	}

	private checkStatus = async () => {
		if (!browser) {
			return
		}
		try {
			this.enabled = await navigator.bluetooth.getAvailability()
			if (this.connected && !this.enabled) {
				// bluetooth was disabled
				this.weightCharacteristic?.removeEventListener('characteristicvaluechanged', this.onWeightUpdate)
				this.connected = false
			}
		} catch (err) {
			console.log(err)
			this.enabled = false
		}
	}

	public static getInstance(): Bluetooth {
		if (!Bluetooth.instance) {
			Bluetooth.instance = new Bluetooth()
		}
		return Bluetooth.instance
	}

	connect = async () => {
		if (!browser || !navigator.bluetooth) {
			return
		}
		// Support both the old python firmware (with device name `mpy-coffee`) and the new rust firmware
		// (with device name `coffee-scale`).
		// The new firmware uses a more appropriate service and characteristic UUIDs, so we can use those to identify
		// the firmware variant.
		const device = await navigator.bluetooth.requestDevice({
			filters: [
				{ name: 'mpy-coffee' },
				{ name: 'coffee-scale' },
				{ services: [0x180F, 0x1815] }, // python firmware
				{ services: [0x180F, 0x181D] }, // rust firmware
			],
		})
		device.addEventListener('gattserverdisconnected', () => {
			this.connected = false
			this.currentWeight = 0
			this.batteryLevel = undefined
		})
		this.server = await device.gatt?.connect()
		this.connected = true

		// Detect firmware version
		try {
			// python firmware
			const service = await this.server?.getPrimaryService(0x1815)
			this.weightCharacteristic = await service?.getCharacteristic(0x2A59)
			this.newFirmware = false
		} catch {
			// rust firmware
			const service = await this.server?.getPrimaryService(0x181D)
			this.weightCharacteristic = await service?.getCharacteristic(0x2A9D)
			this.newFirmware = true
		}
		await this.weightCharacteristic?.startNotifications()
		this.weightCharacteristic?.addEventListener('characteristicvaluechanged', this.onWeightUpdate)

		await this.readBatteryLevel()
	}
}
