<template>
  <div id="start" v-b-tooltip.hover.v-warning :title="canRecord ? null : 'Tare scale to enable recording'">
    <b-button v-if="!recording" :disabled="currentWeight > 0.1" variant="primary" size="sm" @click="onClick">
      <small :style="{ fontSize: '0.5rem' }">
        <b-icon-circle-fill :style="{ verticalAlign: 'inherit' }"></b-icon-circle-fill>
      </small>
      &nbsp;Start recording
    </b-button>
  </div>
</template>

<script>
import { mapState, mapActions } from 'vuex'

export default {
  name: 'StartButton',
  computed: {
    ...mapState(['connected', 'recording', 'currentWeight']),
    canRecord() {
      return !this.recording && this.currentWeight <= 0.1
    }
  },
  methods: {
    ...mapActions(['startRecording']),
    onClick() {
      this.startRecording()
    }
  }
}
</script>

<style scoped lang="scss">
#start {
  position: absolute;
  left: 4rem;
  bottom: 5rem;
}
</style>
