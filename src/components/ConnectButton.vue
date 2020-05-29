<template>
  <b-overlay :show="!connected" no-wrap :style="{ zIndex: 1000 }">
    <template v-slot:overlay>
      <div class="text-center">
        <b-button v-if="btEnabled" variant="primary" size="lg" @click="onClick"
          ><b-icon-power></b-icon-power>&nbsp;Connect to scale
        </b-button>
        <p v-if="!btEnabled" class="h1">
          <b-icon-exclamation-diamond variant="danger"></b-icon-exclamation-diamond>
        </p>
        <p v-if="!btEnabled" class="h2">
          Bluetooth is not available
        </p>
      </div>
    </template>
  </b-overlay>
</template>

<script>
import { mapState, mapActions } from 'vuex'

export default {
  name: 'ConnectButton',
  computed: {
    ...mapState(['btEnabled', 'connected'])
  },
  methods: {
    ...mapActions(['connect']),
    onClick() {
      this.connect().catch((error) => {
        console.log(error)
        this.$bvToast.toast(error.message, {
          title: 'Bluetooth Error'
        })
      })
    }
  }
}
</script>
