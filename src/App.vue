<template>
  <b-container fluid="lg" id="app">
    <h1>Brew</h1>
    <chart :chart-data="datacollection" :styles="chartStyles"></chart>
  </b-container>
</template>

<script>
import Chart from "./components/Chart.vue";

export default {
  name: "App",
  components: {
    Chart
  },
  data() {
    return {
      datacollection: null
    };
  },
  mounted() {
    this.fillData();
  },
  methods: {
    fillData() {
      this.datacollection = {
        // labels: [this.getRandomInt(), this.getRandomInt()],
        datasets: [
          {
            label: "Data One",
            backgroundColor: "#eee",
            fill: true,
            lineTension: 0,
            data: [
              { x: 0, y: this.getRandomInt() },
              { x: 1, y: this.getRandomInt() },
              { x: 3, y: this.getRandomInt() }
            ]
          },
          {
            label: "Data Two",
            backgroundColor: "#528078",
            borderColor: "#528078",
            fill: false,
            lineTension: 0,
            data: [
              { x: 0, y: this.getRandomInt() },
              { x: 1, y: this.getRandomInt() }
            ]
          }
        ]
      };
    },
    getRandomInt() {
      return Math.floor(Math.random() * (50 - 5 + 1)) + 5;
    }
  },
  computed: {
    chartStyles() {
      return {
        height: "calc(100vh - 4rem)",
        position: "relative"
      };
    }
  }
};
</script>

<style lang="scss"></style>
