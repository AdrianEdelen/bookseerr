<script lang="ts">
	import '../app.css';
	import { AppBar } from '@skeletonlabs/skeleton-svelte';
  	import { GitBranch, ArrowLeft, Paperclip, Calendar, CircleUser}  from 'lucide-svelte';
	import HealthCheckButton from '$lib/components/HealthCheckButton.svelte'
	import RequestButton from '$lib/components/RequestButton.svelte';
	let { children } = $props();
</script>




<AppBar>
	<!-- {#snippet lead()}
		<ArrowLeft size={24} />
	{/snippet} -->
	{#snippet trail()}
		<a href="https://github.com/AdrianEdelen/bookseerr" target="_blank" rel="noopener noreferrer" class="hover:opacity-40">
			<GitBranch size={20} />
		</a>
		<Calendar size={20} />
		<CircleUser size={20} />
	{/snippet}

	{#snippet headline()}
		<div class="flex items-center gap-4">
			<h2 class="h2">Bookseerr</h2>
			<RequestButton />

		</div>
	{/snippet}
	<span> Bookseerr </span>
</AppBar>

<hr class="hr" />
	
{@render children()}
	