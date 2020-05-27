<script>
import { Scatter } from 'vue-chartjs'

export default {
  name: 'Chart',
  extends: Scatter,
  //mixins: [reactiveProp],
  props: {
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
      return {
        datasets: [
          {
            label: 'Current Weight',
            backgroundColor: '#528078',
            borderColor: '#528078',
            fill: false,
            showLine: true,
            lineTension: 0,
            pointRadius: 0,
            borderCapStyle: 'round',
            data: this.currentData
          },
          {
            label: 'Target Weight',
            backgroundColor: '#eee',
            borderColor: '#eee',
            fill: true,
            showLine: true,
            cubicInterpolationMode: 'monotone',
            data: [
              { x: 0, y: 0 },
              { x: 5, y: 0 },
              { x: 30, y: 38 },
              { x: 35, y: 38 }
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
