<template>
  <b-form>
    <b-row align-v="center">
      <b-col>
        <h1>Brew</h1>
      </b-col>
      <b-col>
        <b-form-group
          id="coffee-weight"
          description="How much coffee grounds in basket"
          label="Coffee weight"
          label-for="input-coffee-weight"
          label-align="right"
          label-cols
        >
          <b-input-group append="g">
            <b-form-input
              id="input-coffee-weight"
              v-model="coffeeWeight"
              type="number"
              number
            ></b-form-input>
          </b-input-group>
        </b-form-group>
      </b-col>
    </b-row>
  </b-form>
</template>

<script>
export default {}
</script>

<style lang="scss"></style>
