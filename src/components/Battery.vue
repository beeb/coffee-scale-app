<template>
  <div v-if="batteryLevel" id="battery">
    <span>{{ batteryLevel }}%&nbsp;</span>
    <b-icon-battery-full></b-icon-battery-full>
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
  right: 0;
  bottom: 0;
  font-size: 1.3rem;

  span {
    font-size: 0.9rem;
  }
}
</style>
