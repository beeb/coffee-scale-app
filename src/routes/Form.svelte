<script>
  import InputCoffeeWeight from './InputCoffeeWeight.svelte'
  import InputPreinfusion from './InputPreinfusion.svelte'
  import InputRatio from './InputRatio.svelte'
  import InputTotalTime from './InputTotalTime.svelte'
  import ArrowRight from 'virtual:icons/mingcute/arrow-right-line'
</script>

<div class="flex flex-col bg-base-100 p-4 h-full">
  <div class="absolute top-2 right-2">
    <label for="form-drawer" class="btn btn-sm btn-ghost btn-square" aria-label="Close drawer">
      <ArrowRight class="h-6 w-6" />
    </label>
  </div>
  <InputCoffeeWeight />
  <InputRatio />
  <InputPreinfusion />
  <InputTotalTime />
</div>
