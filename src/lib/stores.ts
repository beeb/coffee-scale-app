import { derived, writable } from 'svelte/store'

export const coffeeWeight = writable(18.0)
export const targetRatio = writable(2.0)
export const preInfusion = writable(5.0)
export const totalTime = writable(30.0)

export const chartData = writable([])

export const targetWeight = derived([coffeeWeight, targetRatio], ([$coffeeWeight, $targetRatio]) => {
	return $coffeeWeight * $targetRatio
})
