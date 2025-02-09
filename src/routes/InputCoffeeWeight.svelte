<script>
  import { Scale } from '$lib/scale.svelte'

  const scale = Scale.getInstance()
</script>

<div class="form-control w-full max-w-[12rem]">
  <label class="label" for="coffee-weight">
    <span class="label-text text-sm">Coffee weight</span>
  </label>
  <div class="join">
    <button
      type="button"
      class="btn btn-primary join-item btn-sm"
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
      class="input input-bordered input-sm w-full max-w-xs join-item"
    />
    <div class="join-item bg-base-200 px-3 text-sm inline-flex items-center">g</div>
  </div>
</div>
