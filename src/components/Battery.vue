<template>
  <div v-if="batteryLevel" id="battery">
    <span>{{ batteryLevel }}%&nbsp;</span>
    <b-icon-battery-full v-if="batteryLevel >= 60" :style="{ verticalAlign: 'middle' }"></b-icon-battery-full>
    <b-icon-battery-half
      v-if="batteryLevel < 60 && batteryLevel >= 30"
      :style="{ verticalAlign: 'middle' }"
    ></b-icon-battery-half>
    <b-icon-battery v-if="batteryLevel < 30" variant="danger" :style="{ verticalAlign: 'middle' }"></b-icon-battery>
  </div>
</template>

<script>
import { mapState } from 'vuex'

export default {
  name: 'Battery',
  computed: {
    ...mapState(['batteryLevel'])
  }
}
</script>

<style scoped lang="scss">
#battery {
  position: absolute;
  right: 2.5rem;
  bottom: 0.8rem;
  font-size: 1.3rem;

  span {
    font-size: 0.9rem;
  }
}
</style>
