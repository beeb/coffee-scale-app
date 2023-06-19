<script>
  import '../app.css'
  import Chart from './Chart.svelte'
  import Form from './Form.svelte'
  import Title from './Title.svelte'
  import toast, { Toaster } from 'svelte-french-toast'
  import Settings from 'virtual:icons/mingcute/settings-1-line'
  import StartButton from './StartButton.svelte'
  import Gauge from './Gauge.svelte'
  import { onMount } from 'svelte'
  import { checkStatus } from '$lib/bt'

  onMount(async () => {
    try {
      await checkStatus()
    } catch (e) {
      console.error(e)
      toast.error('Bluetooth Error')
    }
    if ('onavailabilitychanged' in navigator.bluetooth) {
      navigator.bluetooth.addEventListener('availabilitychanged', async () => {
        await checkStatus()
      })
    }
  })
</script>

<div class="w-full h-full max-w-7xl mx-auto relative drawer drawer-end">
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
        <StartButton />
      </div>
      <div class="absolute left-20 top-[calc(50%-5rem)]" style="width: min(15rem, 45vh)">
        <Gauge
          startAngle={-110}
          endAngle={110}
          value={12}
          max={45}
          separatorStep={45 / 4}
          innerRadius={70}
          scaleInterval={0}
        >
          <div class="w-full h-full text-3xl font-bold text-center mt-16">12.00g</div>
        </Gauge>
      </div>
    </div>
  </div>
  <div class="drawer-side">
    <label for="form-drawer" class="drawer-overlay" />
    <Form />
  </div>
</div>
<Toaster />

<svelte:head>
  <title>Coffee Scale</title>
</svelte:head>
