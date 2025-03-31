<script>
    let { children } = $props();
</script>
<span>discover layout</span>

<main class="flex-1 overflow-auto p-6">
    {@render children()}
</main>