import { writable as persistentWritable } from '@macfja/svelte-persistent-store'
import { derived, get, writable } from 'svelte/store'

export type DataPoint = { x: number; y: number }

export const btEnabled = writable(false)
export const btConnected = writable(false)
export const btServer = writable<BluetoothRemoteGATTServer | null>(null)
export const recording = writable(false)
export const startTimeMs = writable(0)

export const coffeeWeight = persistentWritable('li.beeb.coffee.v2.coffeeWeight', 18.0)
export const targetRatio = persistentWritable('li.beeb.coffee.v2.targetRatio', 2.0)
export const preInfusion = persistentWritable('li.beeb.coffee.v2.preInfusion', 5.0)
export const totalTime = persistentWritable('li.beeb.coffee.v2.totalTime', 30.0)

export const currentWeight = writable(0.0)
export const batteryLevel = writable<null | number>(null)

export const chartData = writable<DataPoint[]>([])

export const targetWeight = derived([coffeeWeight, targetRatio], ([$coffeeWeight, $targetRatio]) => {
	return $coffeeWeight * $targetRatio
})

export function startRecording() {
	// Initially, the `startTimeMs` is set to 0 during the pre-infusion stage
	// When coffee starts to drop (threshold 0.5g), we will start to record data points and record the start time
	chartData.set([{ x: 0, y: 0 }])
	recording.set(true)
}

export function recordWeight() {
	if (!get(recording)) {
		return
	}
	const startTime = get(startTimeMs)
	const weight = get(currentWeight)
	const now = new Date().getTime()
	if (startTime === 0 && weight > 0.5) {
		// We reached the threshold weight to exit the pre-infusion stage
		startTimeMs.set(now - get(preInfusion) * 1000)
	} else if (startTime > 0 && weight < -0.1) {
		// End of the shot, we removed the cup from the scale
		// The recording is stopped
		startTimeMs.set(0)
		recording.set(false)
	} else if (startTime > 0) {
		// Recording the shot...
		const elapsed = (now - startTime) / 1000
		chartData.update((data) => {
			data.push({ x: elapsed, y: weight })
			return data
		})
	}
}
