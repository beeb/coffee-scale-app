<template>
  <b-overlay :show="!connected" no-wrap :style="{ zIndex: 1000 }">
    <template v-slot:overlay>
      <div v-if="btEnabled" class="text-center">
        <b-button variant="primary" size="lg" @click="onClick"
          ><b-icon-power></b-icon-power>&nbsp;Connect to scale
        </b-button>
      </div>
      <div v-if="!btEnabled" class="text-center">
        <p class="h1">
          <b-icon-exclamation-diamond variant="danger"></b-icon-exclamation-diamond>
        </p>
        <p class="h2">
          Bluetooth is not available
        </p>
      </div>
    </template>
  </b-overlay>
</template>

<script>
import { mapState, mapActions } from 'vuex'
import StayAwake from 'stayawake.js'

export default {
  name: 'ConnectButton',
  computed: {
    ...mapState(['btEnabled', 'connected'])
  },
  mounted() {
    StayAwake.init()
  },
  methods: {
    ...mapActions(['connect', 'getBatteryLevel']),
    onClick() {
      StayAwake.enable()
      this.connect()
        .then(() => {
          this.getBatteryLevel().catch((error) => {
            console.log(error)
            this.$bvToast.toast(error.message, {
              title: 'Bluetooth Error'
            })
          })
        })
        .catch((error) => {
          console.log(error)
          this.$bvToast.toast(error.message, {
            title: 'Bluetooth Error'
          })
        })
    }
  }
}
</script>
