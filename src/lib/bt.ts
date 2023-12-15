import { browser } from '$app/environment'
import { get } from 'svelte/store'
import { batteryLevel, btConnected, btEnabled, btServer, currentWeight, recordWeight, wakeLock } from './stores'

let weightCharacteristic: BluetoothRemoteGATTCharacteristic | null = null
let newFirmware = false

async function onWeightUpdate(event: Event) {
	if (event.target === null) {
		return
	}
	const dataView = (event.target as BluetoothRemoteGATTCharacteristic).value
	const value = (newFirmware ? dataView?.getInt32(0, false) : dataView?.getInt16(0, false)) ?? 0
	currentWeight.set(value / 100.0)
	await recordWeight()
}

export async function checkBtStatus() {
	if (!browser) {
		return false
	}
	const available = await navigator.bluetooth.getAvailability()
	btEnabled.set(available)
	if (get(btConnected) && !available) {
		weightCharacteristic?.removeEventListener('characteristicvaluechanged', onWeightUpdate)
		btConnected.set(false)
		const wake = get(wakeLock)
		if (wake !== null) {
			await wake.release()
			wakeLock.set(null)
		}
	}
	return available
}

export async function connectBt() {
	if (!browser) {
		return
	}
	const device = await navigator.bluetooth.requestDevice({
		filters: [
			{ name: 'mpy-coffee' },
			{ name: 'coffee-scale' },
			{ services: [parseInt('0x180F'), parseInt('0x1815')] }, // python firmware
			{ services: [parseInt('0x180F'), parseInt('0x181D')] }, // rust firmware
		],
	})
	device.addEventListener('gattserverdisconnected', () => {
		btConnected.set(false)
		currentWeight.set(0.0)
		batteryLevel.set(null)
	})
	const server = await device.gatt?.connect()
	btConnected.set(true)
	btServer.set(server ?? null)

	try {
		// python firmware
		const service = await server?.getPrimaryService(parseInt('0x1815'))
		weightCharacteristic = (await service?.getCharacteristic(parseInt('0x2A59'))) ?? null
		newFirmware = false
	} catch {
		// rust firmware
		const service = await server?.getPrimaryService(parseInt('0x181D'))
		weightCharacteristic = (await service?.getCharacteristic(parseInt('0x2A9D'))) ?? null
		newFirmware = true
	}
	await weightCharacteristic?.startNotifications()
	weightCharacteristic?.addEventListener('characteristicvaluechanged', onWeightUpdate)

	await readBatteryLevel()
	if ('wakeLock' in navigator) {
		try {
			wakeLock.set(await navigator.wakeLock.request('screen'))
		} catch (err) {
			console.error(err)
		}
	}
}

export async function readBatteryLevel() {
	const server = get(btServer)
	if (server === null) {
		return
	}
	const service = await server.getPrimaryService(parseInt('0x180F'))
	const batteryLevelCharacteristic = await service.getCharacteristic(parseInt('0x2A19'))
	const value = (await batteryLevelCharacteristic.readValue()).getUint8(0)
	batteryLevel.set(value)
}
