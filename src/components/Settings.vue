<template>
  <b-form id="params-form" :style="{ width: width }" :class="{ 'white-bg': cols == 4 }">
    <b-row v-if="!recording" align-v="center" :cols="cols">
      <b-col>
        <b-form-group
          id="coffee-weight"
          label="Coffee weight"
          label-for="input-coffee-weight"
          label-size="sm"
          md="auto"
        >
          <b-input-group append="g" size="sm" :style="{ backgroundColor: '#fff' }">
            <b-input-group-prepend>
              <b-button
                v-b-tooltip.hover
                variant="outline-primary"
                title="Insert the value that the scale currently reads"
                :disabled="!connected"
                @click="readWeight"
              >
                Read
              </b-button>
            </b-input-group-prepend>
            <b-form-input
              id="input-coffee-weight"
              v-model.lazy="coffeeWeightBind"
              type="number"
              lazy-formatter
              :formatter="formatter"
              number
            ></b-form-input>
          </b-input-group>
        </b-form-group>
      </b-col>
      <b-col>
        <b-form-group id="target-ratio" label="Target ratio" label-for="input-target-ratio" label-size="sm">
          <b-input-group :append="'= ' + targetWeight.toFixed(2) + 'g'" size="sm">
            <b-form-input
              id="input-target-ratio"
              v-model.lazy="targetRatioBind"
              size="sm"
              type="number"
              lazy-formatter
              :formatter="formatter"
              number
            ></b-form-input>
          </b-input-group>
        </b-form-group>
      </b-col>
      <b-col>
        <b-form-group id="pre-infusion-time" label="Pre-infusion time" label-for="input-pre-infusion" label-size="sm">
          <b-input-group append="s" size="sm">
            <b-form-input
              id="input-pre-infusion"
              v-model.lazy="preInfusionBind"
              type="number"
              lazy-formatter
              :formatter="formatter"
              number
            ></b-form-input>
          </b-input-group>
        </b-form-group>
      </b-col>
      <b-col>
        <b-form-group id="total-time" label="Total time" label-for="input-total-time" label-size="sm">
          <b-input-group append="s" size="sm">
            <b-form-input
              id="input-total-time"
              v-model.lazy="totalTimeBind"
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
import { mapState, mapMutations, mapActions, mapGetters } from 'vuex'

export default {
  data() {
    return {
      cols: 1,
      width: '11rem'
    }
  },
  computed: {
    ...mapState(['connected', 'recording', 'coffeeWeight', 'targetRatio', 'preInfusion', 'totalTime']),
    ...mapGetters(['targetWeight']),
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
    ...mapActions(['readWeight']),
    checkOrientation() {
      let aspectRatio = document.getElementById('app').offsetWidth / window.innerHeight
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

<style lang="scss">
#params-form {
  position: absolute;
  right: 2.5rem;
  bottom: 4rem;
  z-index: 200;
  padding: 1rem;
  border-radius: 0.5rem;

  .form-group {
    margin-bottom: 0;

    .col-form-label-sm {
      font-size: 0.7rem;
      margin-bottom: 0;
    }
  }
}
.white-bg {
  background-color: #ffffff;
}
</style>
