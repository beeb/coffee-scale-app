<script>
  import { Scale } from '$lib/scale.svelte'

  const scale = Scale.getInstance()
</script>

<div class="form-control w-full max-w-[12rem]">
  <label class="label" for="coffee-weight">
    <span class="label-text text-sm">Target ratio</span>
  </label>
  <div class="join">
    <input
      id="coffee-weight"
      type="number"
      bind:value={scale.targetRatio.current}
      min="0"
      step="0.05"
      class="input input-bordered input-sm w-full max-w-xs join-item"
    />
    <div class="join-item bg-base-200 px-3 text-sm inline-flex items-center flex-none">
      = {scale.targetWeight.toFixed(2)}g
    </div>
  </div>
</div>
