<script>
  import '../app.css'
  import Chart from './Chart.svelte'
  import Form from './Form.svelte'
  import Title from './Title.svelte'
  import { Toaster } from 'svelte-french-toast'
</script>

<div class="w-full h-full max-w-7xl mx-auto relative">
  <Chart />
  <div class="absolute max-w-7xl inset-0 z-10">
    <Title />
    <Form />
  </div>
</div>
<Toaster />

<svelte:head>
  <title>Coffee Scale</title>
</svelte:head>
