<template>
  <b-container id="app" fluid="lg">
    <h1>Brew</h1>
    <Chart :current-data="currentData" :styles="chartStyles"></Chart>
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
      currentData: [{ x: 0, y: 0 }],
      chartStyles: {
        height: 'calc(100vh - 5rem)',
        position: 'relative'
      }
    }
  },
  mounted() {
    this.fillData()
    this.counter = 7
    this.interval = setInterval(() => {
      let lastElem = this.currentData[this.currentData.length - 1].y
      let newY = lastElem + Math.random()
      this.currentData.push({ x: this.counter, y: newY })
      this.counter += 1
    }, 1000)
  },
  methods: {
    fillData() {
      this.currentData = [
        { x: 0, y: 0 },
        { x: 5, y: 0.2 },
        { x: 6, y: 1.2 }
      ]
    }
  }
}
</script>

<style lang="scss">
#app {
  margin-top: 1rem;
}
</style>
