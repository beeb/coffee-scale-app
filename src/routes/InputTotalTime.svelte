<script>
  import { totalTime } from '$lib/stores'
</script>

<div class="form-control w-full max-w-[12rem]">
  <label class="label" for="coffee-weight">
    <span class="label-text text-sm">Total time</span>
  </label>
  <div class="join">
    <input
      id="coffee-weight"
      type="number"
      bind:value={$totalTime}
      min="0"
      step="1"
      class="input input-bordered input-sm w-full max-w-xs join-item"
    />
    <div class="join-item bg-base-200 px-3 text-sm inline-flex items-center">s</div>
  </div>
</div>
