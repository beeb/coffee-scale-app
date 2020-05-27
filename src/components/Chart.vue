<script>
import { Scatter, mixins } from 'vue-chartjs'

const { reactiveProp } = mixins

export default {
  name: 'Chart',
  extends: Scatter,
  mixins: [reactiveProp],
  props: {
    chartData: {
      type: Object,
      default() {
        return {}
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
  mounted() {
    this.renderChart(this.chartData, this.options)
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss"></style>
