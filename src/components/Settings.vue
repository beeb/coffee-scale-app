<template>
  <b-form>
    <b-row align-v="center">
      <b-col cols="2" sm="4" md="2" lg="2" xl="3">
        <h1 :style="{ whiteSpace: 'nowrap' }">
          <b-icon-hexagon-fill
            v-b-tooltip.hover
            variant="danger"
            title="Scale is not connected"
          ></b-icon-hexagon-fill
          >&nbsp;
          <span class="d-none d-sm-inline">Brew</span>
        </h1>
      </b-col>
      <b-col>
        <b-row align-v="center" cols="1" cols-md="2">
          <b-col>
            <b-form-group
              id="coffee-weight"
              label="Coffee weight"
              label-for="input-coffee-weight"
              label-align="right"
              md="auto"
              label-cols
            >
              <b-input-group append="g">
                <b-input-group-prepend>
                  <b-button
                    v-b-tooltip.hover
                    variant="outline-primary"
                    title="Insert the value that the scale currently reads"
                  >
                    Read
                  </b-button>
                </b-input-group-prepend>
                <b-form-input
                  id="input-coffee-weight"
                  v-model="coffeeWeight"
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
              label="Target ratio"
              label-for="input-target-ratio"
              label-align="right"
              label-cols
            >
              <b-form-input
                id="input-target-ratio"
                v-model="targetRatio"
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
              label="Pre-infusion time"
              label-for="input-pre-infusion"
              label-align="right"
              label-cols
            >
              <b-input-group append="s">
                <b-form-input
                  id="input-pre-infusion"
                  v-model="preInfusion"
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
              label="Total time"
              label-for="input-total-time"
              label-align="right"
              label-cols
            >
              <b-input-group append="s">
                <b-form-input
                  id="input-total-time"
                  v-model="totalTime"
                  type="number"
                  lazy-formatter
                  :formatter="formatter"
                  number
                ></b-form-input>
              </b-input-group>
            </b-form-group>
          </b-col>
        </b-row>
      </b-col>
    </b-row>
  </b-form>
</template>

<script>
export default {
  data() {
    return {
      coffeeWeight: 16.8,
      targetRatio: 2.5,
      preInfusion: 5.0,
      totalTime: 30.0
    }
  },
  methods: {
    formatter(value) {
      let val = Number.parseFloat(value)
      if (isNaN(val)) {
        return String((1).toFixed(2))
      }
      if (val < 0) {
        return String(-val.toFixed(2))
      }
      return val.toFixed(2)
    }
  }
}
</script>

<style lang="scss"></style>
