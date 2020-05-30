<template>
  <b-container id="app" fluid="lg" :style="{ position: 'relative' }">
    <Logo></Logo>
    <Settings></Settings>
    <Chart :styles="chartStyles"></Chart>
    <Gauge></Gauge>
    <ConnectButton></ConnectButton>
    <StartButton></StartButton>
  </b-container>
</template>

<script>
import Settings from './components/Settings.vue'
import Chart from './components/Chart.vue'
import Logo from './components/Logo.vue'
import Gauge from './components/Gauge.vue'
import ConnectButton from './components/ConnectButton.vue'
import StartButton from './components/StartButton.vue'

import { mapActions } from 'vuex'

export default {
  name: 'App',
  components: {
    Logo,
    Settings,
    Chart,
    Gauge,
    ConnectButton,
    StartButton
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
  mounted() {
    this.checkBtStatus().catch((error) => {
      console.log(error)
      this.$bvToast.toast(error.message, {
        title: 'Bluetooth Error'
      })
    })
    if ('onavailabilitychanged' in navigator.bluetooth) {
      navigator.bluetooth.addEventListener('availabilitychanged', () => {
        this.checkBtStatus()
      })
    }
  },
  methods: {
    ...mapActions(['checkBtStatus'])
  }
}
</script>

<style lang="scss">
#app {
  min-height: 100vh;
}
</style>
