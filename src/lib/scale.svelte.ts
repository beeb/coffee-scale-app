import { browser } from '$app/environment'
import type { Point } from 'chart.js'
import { untrack } from 'svelte'
import { persistedState } from 'svelte-persisted-state'
import type { Bluetooth } from './bt.svelte'

export class Scale {
	private static instance: Scale
	private static bluetooth: Bluetooth

	recording = $state(false)
	startTimeMs = $state(0)

	coffeeWeight = persistedState('li.beeb.coffee.v2.coffeeWeight', 18.0)
	targetRatio = persistedState('li.beeb.coffee.v2.targetRatio', 2.0)
	preInfusion = persistedState('li.beeb.coffee.v2.preInfusion', 5.0)
	totalTime = persistedState('li.beeb.coffee.v2.totalTime', 30.0)

	wakeLock = $state<WakeLockSentinel>()
	chartData = $state<Point[]>([])

	targetWeight = $derived(this.coffeeWeight.value * this.targetRatio.value)

	private constructor(bt: Bluetooth) {
		Scale.bluetooth = bt

		$effect(() => {
			// effect triggered by changes in these values
			this.recording
			const weight = Scale.bluetooth.currentWeight

			untrack(() => {
				if (!this.recording) {
					return
				}
				const now = new Date().getTime()
				if (this.startTimeMs === 0 && weight > 0.5) {
					// We reached the threshold weight to exit the pre-infusion stage
					this.startTimeMs = now - this.preInfusion.value * 1000
				} else if (this.startTimeMs > 0 && weight < -0.1) {
					// End of the shot, we removed the cup from the scale
					// The recording is stopped
					this.stopRecording()
				} else if (this.startTimeMs > 0) {
					// Recording the shot...
					const elapsed = (now - this.startTimeMs) / 1000
					this.chartData.push({ x: elapsed, y: weight })
				}
			})
		})

		$effect(() => {
			if (Scale.bluetooth.connected) {
				this.acquireWakeLock()
			} else {
				this.releaseWakeLock()
			}
		})
	}

	public static init(bluetooth: Bluetooth) {
		if (!Scale.instance) {
			Scale.instance = new Scale(bluetooth)
		}
		return Scale.instance
	}

	public static getInstance(): Scale {
		if (!Scale.instance) {
			throw new Error('Scale not initialized')
		}
		return Scale.instance
	}

	bt = () => Scale.bluetooth

	private acquireWakeLock = async () => {
		if (!browser) {
			return
		}
		if ('wakeLock' in navigator && !this.wakeLock) {
			try {
				this.wakeLock = await navigator.wakeLock.request('screen')
			} catch (err) {
				console.error(err)
			}
		}
	}

	private releaseWakeLock = async () => {
		await this.wakeLock?.release()
		this.wakeLock = undefined
	}

	startRecording = () => {
		// Initially, the `startTimeMs` is set to 0 during the pre-infusion stage
		// When coffee starts to drip (threshold 0.5g), we will start to record data points and record the start time
		this.chartData = [{ x: 0, y: 0 }]
		this.recording = true
	}

	stopRecording = () => {
		this.startTimeMs = 0
		this.recording = false
	}
}
