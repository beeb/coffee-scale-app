<template>
  <b-form id="params-form" :style="{ width: width }">
    <b-row align-v="center" :cols="cols">
      <b-col>
        <b-form-group
          id="coffee-weight"
          description="Coffee weight"
          label-for="input-coffee-weight"
          label-align="right"
          label-size="sm"
          md="auto"
        >
          <b-input-group append="g" size="sm">
            <b-input-group-prepend>
              <b-button
                v-b-tooltip.hover
                variant="outline-primary"
                title="Insert the value that the scale currently reads"
                :style="{ backgroundColor: '#fff' }"
                :disabled="!connected"
              >
                Read
              </b-button>
            </b-input-group-prepend>
            <b-form-input
              id="input-coffee-weight"
              v-model="coffeeWeightBind"
              type="number"
              lazy-formatter
              :formatter="formatter"
              number
            ></b-form-input>
          </b-input-group>
        </b-form-group>
      </b-col>
      <b-col>
        <b-form-group
          id="target-ratio"
          description="Target ratio"
          label-for="input-target-ratio"
          label-align="right"
          label-size="sm"
        >
          <b-form-input
            id="input-target-ratio"
            v-model="targetRatioBind"
            size="sm"
            type="number"
            lazy-formatter
            :formatter="formatter"
            number
          ></b-form-input>
        </b-form-group>
      </b-col>
      <b-col>
        <b-form-group
          id="pre-infusion-time"
          description="Pre-infusion time"
          label-for="input-pre-infusion"
          label-align="right"
          label-size="sm"
        >
          <b-input-group append="s" size="sm">
            <b-form-input
              id="input-pre-infusion"
              v-model="preInfusionBind"
              type="number"
              lazy-formatter
              :formatter="formatter"
              number
            ></b-form-input>
          </b-input-group>
        </b-form-group>
      </b-col>
      <b-col>
        <b-form-group
          id="total-time"
          description="Total time"
          label-for="input-total-time"
          label-align="right"
          label-size="sm"
        >
          <b-input-group append="s" size="sm">
            <b-form-input
              id="input-total-time"
              v-model="totalTimeBind"
              type="number"
              lazy-formatter
              :formatter="formatter"
              number
            ></b-form-input>
          </b-input-group>
        </b-form-group>
      </b-col>
    </b-row>
  </b-form>
</template>

<script>
import { mapState, mapMutations } from 'vuex'

export default {
  data() {
    return {
      cols: 1,
      width: '11rem'
    }
  },
  computed: {
    ...mapState(['connected', 'coffeeWeight', 'targetRatio', 'preInfusion', 'totalTime']),
    coffeeWeightBind: {
      get() {
        return this.coffeeWeight
      },
      set(value) {
        this.setCoffeeWeight({ weight: value })
      }
    },
    targetRatioBind: {
      get() {
        return this.targetRatio
      },
      set(value) {
        this.setTargetRatio({ ratio: value })
      }
    },
    preInfusionBind: {
      get() {
        return this.preInfusion
      },
      set(value) {
        this.setPreInfusion({ time: value })
      }
    },
    totalTimeBind: {
      get() {
        return this.totalTime
      },
      set(value) {
        this.setTotalTime({ time: value })
      }
    }
  },
  mounted() {
    this.checkOrientation()
    window.addEventListener('resize', () => {
      this.checkOrientation()
    })
  },
  methods: {
    ...mapMutations(['setCoffeeWeight', 'setTargetRatio', 'setPreInfusion', 'setTotalTime']),
    checkOrientation() {
      let aspectRatio = window.innerWidth / window.innerHeight
      if (aspectRatio > 3) {
        this.cols = 4
        this.width = '80%'
      } else {
        this.cols = 1
        this.width = '11rem'
      }
    },
    formatter(value, event) {
      let val = Number.parseFloat(value)
      let precision = event.target.id === 'input-total-time' || event.target.id === 'input-pre-infusion' ? 0 : 2
      if (isNaN(val)) {
        return String((1).toFixed(precision))
      }
      if (val < 0) {
        return String((0).toFixed(precision))
      }
      return val.toFixed(precision)
    }
  }
}
</script>

<style scoped lang="scss">
#params-form {
  position: absolute;
  right: 3rem;
  bottom: 4rem;
  z-index: 200;
  padding: 1rem;
}
.form-group {
  margin-bottom: 0.2rem;
}
</style>
