<script lang="ts">
    import { checkHealth } from "$lib/api/health";
    let status: string | null = null;
    async function handleClick() {
        status = await checkHealth();
    }
</script>

<button on:click={handleClick} type="button" class="btn preset-tonal-secondary hover:preset-tonal">Request a book</button>


