<script lang="ts">
    import { checkHealth } from "$lib/api/health";
    let status: string | null = null;
    async function handleClick() {
        status = await checkHealth();
    }
</script>

<button on:click={handleClick} class="border p-2 rounded bg-blue-500 text-white hover:bg-blue-600">
    Check Backend Health
</button>

{#if status}
    <p class="mt-4 text-lg">Health Status: {status}</p>
{/if}
