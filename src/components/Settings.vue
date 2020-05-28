<template>
  <b-form>
    <b-row align-v="center">
      <b-col sm="2" md="1" lg="2" xl="4" class="d-none d-sm-block">
        <h1>Brew</h1>
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
                    >Read</b-button
                  >
                </b-input-group-prepend>
                <b-form-input
                  id="input-coffee-weight"
                  v-model="coffeeWeight"
                  type="number"
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
      preInfusion: 5,
      totalTime: 30
    }
  }
}
</script>

<style lang="scss"></style>
