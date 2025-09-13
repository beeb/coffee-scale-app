<script>
import '../app.css'
import Chart from './Chart.svelte'
import SettingsForm from './SettingsForm.svelte'
import Title from './Title.svelte'
import { Toaster } from 'svelte-french-toast'
import Settings from 'virtual:icons/mingcute/settings-1-line'
import ConnectStartButton from './ConnectStartButton.svelte'
import Gauge from './Gauge.svelte'
import { Scale } from '$lib/scale.svelte'
import BatteryLevel from './BatteryLevel.svelte'

const scale = Scale.getInstance()
</script>

<div class="w-screen h-screen max-w-7xl mx-auto relative drawer drawer-end">
  <input id="form-drawer" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content">
    <Chart />
    <div class="absolute max-w-7xl inset-0">
      <Title />
      <div class="absolute right-10 bottom-20">
        <label for="form-drawer" class="btn btn-neutral drawer-button" aria-label="Open settings drawer">
          <Settings class="h-6 w-6" /> Settings
        </label>
      </div>
      <div class="absolute left-20 bottom-20">
        <ConnectStartButton />
      </div>
      {#if scale.bt.connected}
        <div class="absolute left-20 top-[calc(50%-5rem)]" style="width: min(15rem, 45vh)">
          <Gauge
            startAngle={-110}
            endAngle={110}
            value={scale.bt.currentWeight}
            max={scale.targetWeight}
            separatorStep={scale.targetWeight / 4}
            innerRadius={70}
            scaleInterval={0}
          >
            <div class="w-full h-full text-3xl font-bold text-center mt-16">
              {scale.bt.currentWeight.toFixed(2)}g
            </div>
          </Gauge>
        </div>
      {/if}
      {#if scale.bt.batteryLevel}
        <div class="absolute right-4 bottom-2">
          <BatteryLevel level={scale.bt.batteryLevel} />
        </div>
      {/if}
    </div>
  </div>
  <div class="drawer-side">
    <label for="form-drawer" class="drawer-overlay"></label>
    <SettingsForm />
  </div>
</div>
<Toaster />

<svelte:head>
  <title>Coffee Scale</title>
</svelte:head>
