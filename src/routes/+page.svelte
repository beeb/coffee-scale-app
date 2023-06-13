<script>
  import '../app.css'
  import Chart from './Chart.svelte'
  import Title from './Title.svelte'
</script>

<div class="w-full h-full max-w-7xl mx-auto relative">
  <Chart />
  <div class="absolute max-w-7xl inset-0 z-10">
    <Title />
  </div>
</div>

<svelte:head>
  <title>Coffee Scale</title>
</svelte:head>
