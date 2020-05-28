<script>
import { Scatter } from 'vue-chartjs'

export default {
  name: 'Chart',
  extends: Scatter,
  //mixins: [reactiveProp],
  props: {
    coffeeWeight: {
      type: Number,
      default: 17
    },
    preInfusion: {
      type: Number,
      default: 5
    },
    totalTime: {
      type: Number,
      default: 30
    },
    targetRatio: {
      type: Number,
      default: 2.5
    },
    currentData: {
      type: Array,
      default() {
        return [{ x: 0, y: 0 }]
      }
    }
  },
  data() {
    return {
      options: {
        responsive: true,
        maintainAspectRatio: false,
        events: [],
        legend: {
          position: 'bottom'
        },
        scales: {
          xAxes: [
            {
              scaleLabel: { display: true, labelString: 'Time (s)' },
              ticks: { beginAtZero: true, stepSize: 5 }
            }
          ],
          yAxes: [
            {
              scaleLabel: { display: true, labelString: 'Weight (g)' },
              ticks: { beginAtZero: true }
            }
          ]
        },
        animation: {
          duration: 50
        }
      }
    }
  },
  computed: {
    chartData() {
      let targetWeight = this.targetRatio * this.coffeeWeight
      return {
        datasets: [
          {
            label: 'Current',
            backgroundColor: '#63e792',
            borderColor: '#63e792',
            fill: false,
            showLine: true,
            lineTension: 0,
            pointRadius: 0,
            borderCapStyle: 'round',
            data: this.currentData
          },
          {
            label: 'Target',
            backgroundColor: '#fdfbf788',
            borderColor: '#555555',
            fill: true,
            showLine: true,
            cubicInterpolationMode: 'monotone',
            pointRadius: 0,
            data: [
              { x: 0, y: 0 },
              { x: this.preInfusion, y: 0 },
              { x: this.totalTime, y: targetWeight },
              { x: this.totalTime + 10, y: targetWeight }
            ]
          }
        ]
      }
    }
  },
  watch: {
    chartData(newChartData) {
      let chart = this.$data._chart
      if (chart === undefined || chart.data.datasets.length < 2) {
        return
      }
      chart.data.datasets[0]['data'] = newChartData.datasets[0]['data']
      chart.data.datasets[1]['data'] = newChartData.datasets[1]['data']
      chart.update()
      this.$emit('chart:update')
    }
  },
  mounted() {
    this.renderChart(this.chartData, this.options)
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss"></style>
