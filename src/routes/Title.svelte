<script>
import Hexagon from 'virtual:icons/mingcute/hexagon-fill'
import { Scale } from '$lib/scale.svelte'

const scale = Scale.getInstance()

const hexagonClass = $derived(scale.bt.enabled ? (scale.bt.connected ? 'text-primary' : 'text-warning') : 'text-error')
</script>

<div class="absolute left-20 top-10">
  <h1 class="text-3xl md:text-4xl lg:text-5xl font-medium inline-flex items-center gap-2">
    <span class={hexagonClass} class:animate-pulse={scale.recording}>
      <Hexagon />
    </span>
    Brew
  </h1>
</div>
