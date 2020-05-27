<script>
import { Scatter, mixins } from 'vue-chartjs'

const { reactiveProp } = mixins

export default {
  name: 'Chart',
  extends: Scatter,
  mixins: [reactiveProp],
  data() {
    return {
      options: {
        responsive: true,
        maintainAspectRatio: false,
        showLines: true,
        scales: {
          xAxes: [{ ticks: { beginAtZero: true, stepSize: 5 } }],
          yAxes: [{ ticks: { beginAtZero: true } }]
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
