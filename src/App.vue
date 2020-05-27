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
        height: 'calc(100vh - 4rem)',
        position: 'relative'
      }
    }
  },
  mounted() {
    this.fillData()
  },
  methods: {
    fillData() {
      this.datacollection = {
        // labels: [this.getRandomInt(), this.getRandomInt()],
        datasets: [
          {
            label: 'Target',
            backgroundColor: '#eee',
            borderColor: '#eee',
            fill: true,
            showLine: true,
            lineTension: 0,
            data: [
              { x: 0, y: 0 },
              { x: 5, y: 0 },
              { x: 30, y: 38 }
            ]
          }
        ]
      }
    },
    getRandomInt() {
      return Math.floor(Math.random() * (50 - 5 + 1)) + 5
    }
  }
}
</script>

<style lang="scss"></style>
