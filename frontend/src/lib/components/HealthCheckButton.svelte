<script lang="ts">
    import { checkHealth } from "$lib/api/health";
    let status: string | null = null;
    async function handleClick() {
        status = await checkHealth();
    }
</script>

<!-- <div class="flex flex-col items-center gap-4"> -->
    <button on:click={handleClick} type="button" class="btn preset-tonal-primary hover:preset-tonal">Check Backend Health</button>
<!-- </div> -->

{#if status}
    <p class="mt-4 text-lg">Health Status: {status}</p>
{/if}

