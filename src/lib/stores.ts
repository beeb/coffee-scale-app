import { writable as persistentWritable } from '@macfja/svelte-persistent-store'
import { type Writable, derived, writable } from 'svelte/store'

export const btEnabled = writable(false)
export const btConnected = writable(false)
export const recording = writable(false)
export const startTimeMs = writable(0)

export const coffeeWeight = persistentWritable('li.beeb.coffee.v2.coffeeWeight', 18.0)
export const targetRatio = persistentWritable('li.beeb.coffee.v2.targetRatio', 2.0)
export const preInfusion = persistentWritable('li.beeb.coffee.v2.preInfusion', 5.0)
export const totalTime = persistentWritable('li.beeb.coffee.v2.totalTime', 30.0)

export const currentWeight = writable(0.0)
export const batteryLevel: Writable<null | number> = writable(null)

export const chartData = writable([])

export const targetWeight = derived([coffeeWeight, targetRatio], ([$coffeeWeight, $targetRatio]) => {
	return $coffeeWeight * $targetRatio
})
