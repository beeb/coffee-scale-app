<script lang="ts">
  import { preInfusion, totalTime, targetWeight, chartData } from '$lib/stores'
  import { Chart, registerables, type ChartItem } from 'chart.js'

  let chart: Chart | null = null
  let chartRef: ChartItem

  const targetDataset = $derived({
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
  })

  const chartDataset = $derived({
    label: 'Target',
    backgroundColor: '#63e792',
    borderColor: '#63e792',
    fill: false,
    showLine: true,
    lineTension: 0,
    pointRadius: 0,
    borderCapStyle: 'round',
    data: $chartData
  })

  $effect(() => {
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

  $effect(() => {
    if (!chart) return

    chart.data.datasets[0] = chartDataset
    chart.data.datasets[1] = targetDataset
    chart.update('none')
  })
</script>

<div class="p-2 w-full h-screen relative">
  <canvas bind:this={chartRef}></canvas>
</div>
