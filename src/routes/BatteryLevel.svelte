<script lang="ts">
import BatteryFull from 'virtual:icons/mingcute/battery-4-line'
import Battery75 from 'virtual:icons/mingcute/battery-3-line'
import Battery50 from 'virtual:icons/mingcute/battery-2-line'
import Battery25 from 'virtual:icons/mingcute/battery-1-line'
import BatteryEmpty from 'virtual:icons/mingcute/battery-line'

interface Props {
	level?: number
}

const { level = 0 }: Props = $props()

const classes = 'w-5 h-5'
</script>

<div class="flex text-sm items-center gap-2">
  {#if level >= 90}
    <BatteryFull class={classes} />
  {:else if level >= 70}
    <Battery75 class={classes} />
  {:else if level >= 45}
    <Battery50 class={classes} />
  {:else if level >= 20}
    <Battery25 class={classes} />
  {:else}
    <BatteryEmpty class={`${classes} text-error`} />
  {/if}
  {level}%
</div>
