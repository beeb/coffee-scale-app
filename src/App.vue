<template>
  <b-container id="app" fluid="lg">
    <Settings></Settings>
    <Chart
      :coffee-weight="17"
      :pre-infusion="5"
      :total-time="30"
      :target-ratio="2.5"
      :current-data="currentData"
      :styles="chartStyles"
    ></Chart>
  </b-container>
</template>

<script>
import Settings from './components/Settings.vue'
import Chart from './components/Chart.vue'

export default {
  name: 'App',
  components: {
    Settings,
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
    this.counter = 6.1
    this.interval = setInterval(() => {
      let lastElem = this.currentData[this.currentData.length - 1].y
      let newY = lastElem + Math.random() * 0.2
      this.currentData.push({ x: this.counter, y: newY })
      this.counter += 0.1
    }, 100)
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
