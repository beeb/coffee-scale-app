<script>
import { Scatter } from 'vue-chartjs'

import { mapState, mapGetters } from 'vuex'

export default {
  name: 'Chart',
  extends: Scatter,
  data() {
    return {
      options: {
        responsive: true,
        maintainAspectRatio: false,
        events: [],
        legend: {
          display: false,
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
    ...mapState(['preInfusion', 'totalTime', 'currentData']),
    ...mapGetters(['targetWeight']),
    chartData() {
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
              { x: Math.max(0.93 * this.totalTime, this.preInfusion), y: 0.95 * this.targetWeight },
              { x: this.totalTime, y: this.targetWeight },
              { x: this.totalTime + 10, y: this.targetWeight }
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
