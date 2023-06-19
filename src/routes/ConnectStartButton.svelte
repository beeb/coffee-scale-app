<script lang="ts">
  import { btConnected, btEnabled, currentWeight, recording, startRecording, stopRecording } from '$lib/stores'
  import Record from 'virtual:icons/mingcute/record-mail-line'
  import Link from 'virtual:icons/mingcute/link-line'
  import Cross from 'virtual:icons/mingcute/close-circle-line'
  import Warning from 'virtual:icons/mingcute/warning-line'
  import Stop from 'virtual:icons/mingcute/stop-circle-fill'
  import { connectBt } from '$lib/bt'
  import toast from 'svelte-french-toast'

  const connect = async () => {
    try {
      await connectBt()
    } catch (e) {
      console.error(e)
      const error = e as Error
      toast.error(`Bluetooth Error: ${error.message}`)
    }
  }

  $: canRecord = $currentWeight >= -0.1 && $currentWeight <= 0.1
</script>

{#if $btEnabled && $btConnected}
  {#if $recording}
    <button type="button" class="btn btn-primary btn-outline btn-sm sm:btn-md" on:click={stopRecording}>
      <Stop class="h-6 w-6" /> Stop Recording
    </button>
  {:else}
    <div
      class="tooltip tooltip-right tooltip-open"
      class:tooltip-open={!canRecord}
      data-tip="The scale needs to be tared to start the recording"
    >
      <button
        type="button"
        class="btn btn-primary btn-sm sm:btn-md"
        class:btn-disabled={!canRecord}
        on:click={startRecording}
      >
        <Record class="h-6 w-6" /> Start Recording
      </button>
    </div>
  {/if}
{:else if $btEnabled}
  <button type="button" class="btn btn-warning btn-sm sm:btn-md" on:click={connect}>
    <Link class="h-6 w-6" /> Connect to Scale
  </button>
{:else}
  <div class="alert alert-error">
    <Cross class="h-6 w-6" />
    <span>
      Bluetooth not available. Try enabling Bluetooth or switching to a
      <a class="link" href="https://developer.mozilla.org/en-US/docs/Web/API/Web_Bluetooth_API#browser_compatibility">
        supported browser
      </a>
    </span>
  </div>
{/if}
