<script>
import '../app.css'
import { Bluetooth } from '$lib/bt.svelte'
import { Scale } from '$lib/scale.svelte'

const { children } = $props()

Scale.init(Bluetooth.getInstance())
</script>

{@render children()}
