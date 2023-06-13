<script lang="ts">
  import { preInfusion, totalTime, targetWeight, chartData } from '$lib/stores'
  import { Chart, registerables, type ChartItem } from 'chart.js'
  import { afterUpdate, onMount } from 'svelte'

  let chart: Chart | null = null
  let chartRef: ChartItem

  $: targetDataset = {
    label: 'Target',
    backgroundColor: '#fdfbf788',
    borderColor: '#555555',
    fill: true,
    showLine: true,
    lineTension: 0.25,
    pointRadius: 0,
    data: [
      { x: 0, y: 0 },
      { x: $preInfusion, y: 0 },
      {
        x: Math.max($preInfusion + 0.125 * ($totalTime - $preInfusion), $preInfusion),
        y: 0.05 * $targetWeight
      },
      {
        x: Math.max($preInfusion + 0.25 * ($totalTime - $preInfusion), $preInfusion),
        y: 0.15 * $targetWeight
      },
      {
        x: Math.max($preInfusion + 0.97 * ($totalTime - $preInfusion), $preInfusion),
        y: 0.98 * $targetWeight
      },
      { x: Math.max($totalTime, $preInfusion), y: $targetWeight },
      { x: $totalTime + 10, y: $targetWeight }
    ]
  }

  $: chartDataset = {
    label: 'Target',
    backgroundColor: '#fdfbf788',
    borderColor: '#555555',
    fill: true,
    showLine: true,
    lineTension: 0.25,
    pointRadius: 0,
    data: $chartData
  }

  onMount(() => {
    Chart.register(...registerables)
    chart = new Chart(chartRef, {
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
      chart?.destroy()
      chart = null
    }
  })

  afterUpdate(() => {
    if (!chart) return

    chart.data.datasets[0] = targetDataset
    chart.data.datasets[1] = chartDataset
    chart.update('none')
  })
</script>

<div class="p-4 w-full h-full">
  <canvas bind:this={chartRef} />
</div>
