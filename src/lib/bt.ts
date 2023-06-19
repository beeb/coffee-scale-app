import { browser } from '$app/environment'
import { batteryLevel, btConnected, btEnabled, btServer, currentWeight } from './stores'
import { get } from 'svelte/store'

export async function checkStatus() {
	if (!browser) {
		return false
	}
	const available = await navigator.bluetooth.getAvailability()
	btEnabled.set(available)
	if (get(btConnected) && !available) {
		btConnected.set(false)
	}
	return available
}

export async function connect() {
	if (!browser) {
		return
	}
	const device = await navigator.bluetooth.requestDevice({
		filters: [{ name: 'mpy-coffee' }, { services: [parseInt('0x1815'), parseInt('0x180F')] }],
	})
	device.addEventListener('gattserverdisconnected', () => {
		btConnected.set(false)
		currentWeight.set(0.0)
		batteryLevel.set(null)
	})
	const server = await device.gatt?.connect()
	btConnected.set(true)
	btServer.set(server ?? null)
	const service = await server?.getPrimaryService(parseInt('0x1815'))
	const weight = await service?.getCharacteristic(parseInt('0x2A59'))
	await weight?.startNotifications()
	weight?.addEventListener('characteristicvaluechanged', (event) => {
		if (event.target === null) {
			return
		}
		const value = (event.target as BluetoothRemoteGATTCharacteristic).value?.getInt16(0, false) ?? 0
		currentWeight.set(value / 100.0)
	})
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
