<script>
import { Scale } from '$lib/scale.svelte'

const scale = Scale.getInstance()
</script>

<fieldset class="fieldset max-w-[13rem]">
  <legend class="fieldset-legend">Total time</legend>
  <div class="join">
    <input
      id="coffee-weight"
      type="number"
      bind:value={scale.totalTime.current}
      min="0"
      step="1"
      class="input join-item"
    />
    <div class="join-item bg-base-200 px-3 inline-flex items-center">s</div>
  </div>
</fieldset>
