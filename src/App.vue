<template>
  <b-container id="app" fluid="lg">
    <h1>Brew</h1>
    <Chart :chart-data="datacollection" :styles="chartStyles"></Chart>
  </b-container>
</template>

<script>
import Chart from './components/Chart.vue'

export default {
  name: 'App',
  components: {
    Chart
  },
  data() {
    return {
      datacollection: null
    }
  },
  computed: {
    chartStyles() {
      return {
        height: 'calc(100vh - 5rem)',
        position: 'relative'
      }
    }
  },
  mounted() {
    this.fillData()
    this.counter = 7
    this.interval = setInterval(() => {
      let datacollection = { ...this.datacollection }
      let data = this.datacollection.datasets[0].data
      let lastElem = data[data.length - 1].y
      let newY = lastElem + Math.random()
      datacollection.datasets[0].data.push({ x: this.counter, y: newY })
      this.counter += 1
      this.datacollection = datacollection
    }, 1000)
  },
  methods: {
    fillData() {
      this.datacollection = {
        // labels: [this.getRandomInt(), this.getRandomInt()],
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
            data: [
              { x: 0, y: 0 },
              { x: 5, y: 0.2 },
              { x: 6, y: 1.2 }
            ]
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
  }
}
</script>

<style lang="scss">
#app {
  margin-top: 1rem;
}
</style>
