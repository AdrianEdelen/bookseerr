<script lang="ts">
	import '../app.css';
	import { AppBar, Navigation } from '@skeletonlabs/skeleton-svelte';
	import { GitBranch, ArrowLeft, Paperclip, Calendar, CircleUser,
		Image, Music, Video, Folder, Menu
	}  from 'lucide-svelte';
	import RequestButton from '$lib/components/RequestButton.svelte';

	let { children } = $props();
	let value = $state('files');
	let isExpansed = $state(true);
	function toggleExpanded() {
    	isExpansed = !isExpansed;
  	}
</script>




<AppBar> 
	{#snippet trail()}
		<a href="https://github.com/AdrianEdelen/bookseerr" target="_blank" rel="noopener noreferrer" class="hover:opacity-40">
			<GitBranch size={20} />
		</a>
		<Calendar size={20} />
		<CircleUser size={20} />
	{/snippet}
<!-- 
	{#snippet headline()}
		<div class="flex items-center gap-4">
			<h2 class="h2">Bookseerr</h2>
			<RequestButton />

		</div>
	{/snippet} -->
	<div class="flex items-center justify-between w-full gap-4">
		<!-- Left: Title + Search -->
		<div class="flex items-center gap-4 flex-grow">
			<span class="text-xl font-semibold">Bookseerr</span>
	
			<!-- Optional Search Bar -->
			<input
				type="text"
				placeholder="Search books..."
				class="input input-sm w-full max-w-xs"
			/>
		</div>
	
		<!-- Right: Request Button -->
		<RequestButton />
	</div>
</AppBar>
<hr class="hr" />

<div class="card border-surface-100-900 grid h-[640px] w-full grid-cols-[auto_1fr] border-[1px]">
	<!-- Component -->
	<Navigation.Rail expanded={isExpansed}>
		{#snippet header()}
			<Navigation.Tile labelExpanded="Menu" onclick={toggleExpanded} title="Toggle Menu Width">
				<Menu />
			</Navigation.Tile>
		{/snippet}
		{#snippet tiles()}
			<Navigation.Tile id="files" label="Files"><Folder /></Navigation.Tile>
			<Navigation.Tile id="images" label="Images"><Image /></Navigation.Tile>
			<Navigation.Tile id="music" label="Music"><Music /></Navigation.Tile>
			<Navigation.Tile id="videos" label="Videos"><Video /></Navigation.Tile>
	  	{/snippet}
	</Navigation.Rail>
	<!-- Content -->
	<div class="flex items-center justify-center">
	  <pre class="pre">value: {value}</pre>
	</div>
  </div>
	
{@render children()}
	