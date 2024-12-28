<script lang="ts">
  import { Scale } from '$lib/scale.svelte'
  import { Chart, type ChartDataset, registerables } from 'chart.js'

  const scale = Scale.getInstance()

  let chartNode: HTMLCanvasElement
  let chartObject: Chart<'scatter'> | undefined

  const targetDataset = $derived<ChartDataset<'scatter'>>({
    label: 'Target',
    backgroundColor: '#fdfbf788',
    borderColor: '#555555',
    fill: true,
    showLine: true,
    tension: 0.25,
    pointRadius: 0,
    pointBorderWidth: undefined,
    data: [
      { x: 0, y: 0 },
      { x: scale.preInfusion.value, y: 0 },
      {
        x: Math.max(
          scale.preInfusion.value + 0.125 * (scale.totalTime.value - scale.preInfusion.value),
          scale.preInfusion.value
        ),
        y: 0.05 * scale.targetWeight
      },
      {
        x: Math.max(
          scale.preInfusion.value + 0.25 * (scale.totalTime.value - scale.preInfusion.value),
          scale.preInfusion.value
        ),
        y: 0.15 * scale.targetWeight
      },
      {
        x: Math.max(
          scale.preInfusion.value + 0.97 * (scale.totalTime.value - scale.preInfusion.value),
          scale.preInfusion.value
        ),
        y: 0.98 * scale.targetWeight
      },
      { x: Math.max(scale.totalTime.value, scale.preInfusion.value), y: scale.targetWeight },
      { x: scale.totalTime.value + 10, y: scale.targetWeight }
    ]
  })

  const chartDataset = $derived<ChartDataset<'scatter'>>({
    label: 'Target',
    backgroundColor: '#63e792',
    borderColor: '#63e792',
    fill: false,
    showLine: true,
    tension: 0,
    pointRadius: 0,
    pointBorderWidth: undefined,
    borderCapStyle: 'round',
    data: scale.chartData
  })

  $effect(() => {
    Chart.register(...registerables)
    chartObject = new Chart(chartNode, {
      type: 'scatter',
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          x: {
            title: { display: true, text: 'Time (s)' },
            ticks: {
              stepSize: 5
            },
            beginAtZero: true
          },
          y: {
            title: { display: true, text: 'Weight (g)' }
          }
        },
        plugins: {
          legend: { display: false }
        }
      },
      data: {
        datasets: []
      }
    })

    return () => {
      chartObject?.destroy()
      chartObject = undefined
    }
  })

  $effect(() => {
    if (!chartObject) return
    chartObject.data.datasets[0] = $state.snapshot(chartDataset) as ChartDataset<'scatter'>
    chartObject.data.datasets[1] = $state.snapshot(targetDataset) as ChartDataset<'scatter'>
    chartObject.update('none')
  })
</script>

<div class="p-2 w-full h-screen relative">
  <canvas bind:this={chartNode}></canvas>
</div>
