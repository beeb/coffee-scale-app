<script>
  import '../app.css'
  import Chart from './Chart.svelte'
  import Form from './Form.svelte'
  import Title from './Title.svelte'
  import { Toaster } from 'svelte-french-toast'
  import Settings from 'virtual:icons/mingcute/settings-1-line'
</script>

<div class="w-full h-full max-w-7xl mx-auto relative drawer drawer-end">
  <input id="form-drawer" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content">
    <Chart />
    <div class="absolute max-w-7xl inset-0">
      <Title />
      <div class="absolute right-10 top-10">
        <label for="form-drawer" class="btn btn-neutral drawer-button btn-square" aria-label="Open drawer">
          <Settings class="h-6 w-6" />
        </label>
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
