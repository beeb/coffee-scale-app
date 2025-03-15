<script>
  import { Scale } from '$lib/scale.svelte'

  const scale = Scale.getInstance()
</script>

<fieldset class="fieldset max-w-[13rem]">
  <legend class="fieldset-legend">Coffee weight</legend>
  <div class="join">
    <button
      type="button"
      class="btn btn-primary join-item"
      disabled={!scale.bt.connected}
      onclick={() => {
        scale.coffeeWeight.current = scale.bt.currentWeight
      }}
    >
      Read
    </button>
    <input
      id="coffee-weight"
      type="number"
      bind:value={scale.coffeeWeight.current}
      min="0"
      step="0.5"
      class="input join-item"
    />
    <div class="join-item bg-base-200 px-3 inline-flex items-center">g</div>
  </div>
</fieldset>
