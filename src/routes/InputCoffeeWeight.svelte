<script>
  import { coffeeWeight, currentWeight, btConnected } from '$lib/stores'
</script>

<div class="form-control w-full max-w-[12rem]">
  <label class="label" for="coffee-weight">
    <span class="label-text text-sm">Coffee weight</span>
  </label>
  <div class="join">
    <button
      type="button"
      class="btn btn-primary join-item btn-sm"
      disabled={!$btConnected}
      onclick={() => {
        $coffeeWeight = $currentWeight
      }}
    >
      Read
    </button>
    <input
      id="coffee-weight"
      type="number"
      bind:value={$coffeeWeight}
      min="0"
      step="0.5"
      class="input input-bordered input-sm w-full max-w-xs join-item"
    />
    <div class="join-item bg-base-200 px-3 text-sm inline-flex items-center">g</div>
  </div>
</div>
