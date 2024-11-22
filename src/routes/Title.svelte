<script>
  import { btConnected, btEnabled, recording } from '$lib/stores'
  import Hexagon from 'virtual:icons/mingcute/hexagon-fill'

  const hexagonClass = $derived($btEnabled ? ($btConnected ? 'text-primary' : 'text-warning') : 'text-error')
</script>

<div class="absolute left-20 top-10">
  <h1 class="text-3xl md:text-4xl lg:text-5xl font-medium inline-flex items-center gap-2">
    <span class={hexagonClass} class:animate-pulse={$recording}>
      <Hexagon />
    </span>
    Brew
  </h1>
</div>
