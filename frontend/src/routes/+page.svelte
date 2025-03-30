<script lang="ts">
    let status: string | null = null;

    async function checkHealth() {
        try {
            const res = await fetch('http://localhost:3000/health');
            if (!res.ok) throw new Error('Failed to fetch');
            status = await res.text();
        } catch (err) {
            status = 'Error connecting to backend';
        }
    }
</script>
<button on:click={checkHealth} class="border p-2 rounded bg0blue-500 text-white hover:bg-blue-600">
    Check Backend Health
</button>

{#if status}
    <p class="mt-4 text-lg">Health Status: {status}</p>
{/if}

