<template>
  <div id="logo-status">
    <h1>
      <b-icon-hexagon-fill v-b-tooltip.hover variant="danger" title="Scale is not connected"></b-icon-hexagon-fill
      >&nbsp;Brew
    </h1>
  </div>
</template>

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
