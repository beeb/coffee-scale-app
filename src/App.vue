<template>
  <b-container id="app" fluid="lg" :style="{ position: 'relative' }" @click="enableNoSleep">
    <Logo :connected="connected"></Logo>
    <Settings></Settings>
    <Chart :styles="chartStyles"></Chart>
  </b-container>
</template>

<script>
import StayAwake from 'stayawake.js'
import Settings from './components/Settings.vue'
import Chart from './components/Chart.vue'
import Logo from './components/Logo.vue'

import { mapState, mapActions } from 'vuex'

export default {
  name: 'App',
  components: {
    Logo,
    Settings,
    Chart
  },
  data() {
    return {
      chartStyles: {
        height: 'calc(100vh - 2rem)',
        width: 'calc(100% - 2rem)',
        minHeight: '300px',
        position: 'absolute',
        top: '1rem',
        left: '0'
      }
    }
  },
  computed: {
    ...mapState(['connected'])
  },
  mounted() {
    StayAwake.init()
    this.connect()
  },
  methods: {
    ...mapActions(['connect']),
    enableNoSleep() {
      StayAwake.enable()
    },
    disableNoSleep() {
      StayAwake.disable()
    }
  }
}
</script>

<style lang="scss">
#app {
  min-height: 100vh;
}
</style>
