<script>
  import { Scale } from '$lib/scale.svelte'

  const scale = Scale.getInstance()
</script>

<fieldset class="fieldset max-w-[13rem]">
  <legend class="fieldset-legend">Target ratio</legend>
  <div class="join">
    <input
      id="coffee-weight"
      type="number"
      bind:value={scale.targetRatio.current}
      min="0"
      step="0.05"
      class="input join-item"
    />
    <div class="join-item bg-base-200 px-3 inline-flex items-center flex-none">
      = {scale.targetWeight.toFixed(2)}g
    </div>
  </div>
</fieldset>
