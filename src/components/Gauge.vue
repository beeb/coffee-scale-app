<template>
  <div id="gauge">
    <VueSvgGauge
      v-if="connected"
      :start-angle="-110"
      :end-angle="110"
      :value="currentWeight"
      :min="0"
      :max="targetWeight"
      :separator-step="targetWeight / 4"
      :gauge-color="[
        { offset: 0, color: '#42b983' },
        { offset: 100, color: '#f66' }
      ]"
      :transition-duration="100"
      :inner-radius="70"
      :scale-interval="0"
    >
      <div class="inner-text">
        <span>{{ currentWeight.toFixed(1) }}g</span>
      </div>
    </VueSvgGauge>
  </div>
</template>

<script>
import { mapState, mapGetters } from 'vuex'
import { VueSvgGauge } from 'vue-svg-gauge'

export default {
  components: {
    VueSvgGauge
  },
  computed: {
    ...mapState(['connected', 'currentWeight']),
    ...mapGetters(['targetWeight'])
  }
}
</script>

<style scoped lang="scss">
#gauge {
  position: absolute;
  left: 4rem;
  top: 6rem;
  width: 9rem;
  height: 9rem;
}
.inner-text {
  height: 100%;
  width: 100%;
  text-align: center;
  margin-top: 4rem;

  span {
    max-width: 5rem;
    font-weight: bold;
    font-size: 2rem;
  }
}
</style>
