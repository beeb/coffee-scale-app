<template>
  <div id="logo-status">
    <h1>
      <b-icon-hexagon-fill v-b-tooltip.hover :variant="iconVariant" :title="tooltipMessage"></b-icon-hexagon-fill
      >&nbsp;Brew
    </h1>
  </div>
</template>

<script>
export default {
  name: 'Logo',
  props: {
    connected: {
      type: Boolean,
      default: false
    }
  },
  computed: {
    iconVariant() {
      return this.connected ? 'primary' : 'danger'
    },
    tooltipMessage() {
      return this.connected ? 'Scale is connected' : 'Scale is NOT connected'
    }
  }
}
</script>

<style scoped lang="scss">
#logo-status {
  position: absolute;
  left: 3.8rem;
  top: 1.8rem;
  z-index: 100;
  background-color: #fff;
  border-radius: 0.2rem;
  padding: 0.5rem;
}
</style>
