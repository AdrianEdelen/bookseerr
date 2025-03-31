<script lang="ts">
	import '../app.css';
	import { AppBar, Navigation } from '@skeletonlabs/skeleton-svelte';
	import {
		GitBranch,
		ArrowLeft,
		Paperclip,
		Calendar,
		CircleUser,
		Image,
		Music,
		Video,
		Folder,
		Menu,
		BookUser,
		Settings
	} from 'lucide-svelte';
	import RequestButton from '$lib/components/RequestButton.svelte';

	let { children } = $props();
	let value = $state('files');
	let isExpansed = $state(true);
</script>

<div class="flex h-screen flex-col">
	<AppBar>
		{#snippet trail()}
			<div class="flex h-full items-center gap-4">
				<a
					href="https://github.com/AdrianEdelen/bookseerr"
					target="_blank"
					rel="noopener noreferrer"
					class="hover:opacity-40"
				>
					<GitBranch size={20} />
				</a>
				<Calendar size={20} />
				<CircleUser size={20} />
			</div>
		{/snippet}

		<div class="flex w-full items-center justify-between gap-4">
			<div class="flex flex-grow items-center gap-4">
				<span class="text-xl font-semibold">Bookseerr</span>
				<input type="text" placeholder="Search books..." class="input input-sm w-full max-w-xs" />
			</div>
			<RequestButton />
		</div>
	</AppBar>

	<hr class="hr" />

	<div class="flex flex-1 overflow-hidden">
		<div class="bg-surface-2 border-border border-r">
			<Navigation.Rail expanded={isExpansed}>
				{#snippet tiles()}
					<Navigation.Tile id="userRequests" labelExpanded="Your Requests" href="/requests">
						<Folder size={20} />
					</Navigation.Tile>
					<Navigation.Tile id="pendingRequests" labelExpanded="Pending Requests">
						<Video size={20} />
					</Navigation.Tile>
					<Navigation.Tile id="discover" labelExpanded="Discover" href="/files">
						<Music size={20} />
					</Navigation.Tile>
					<Navigation.Tile id="admin" labelExpanded="Admin" href="/admin">
						<Image size={20} />
					</Navigation.Tile>
					
				{/snippet}
				{#snippet footer()}
					<Navigation.Tile id="info" labelExpanded="Settings">
						<Settings />
					</Navigation.Tile>
				{/snippet}
			</Navigation.Rail>
		</div>

		<main class="flex-1 overflow-auto p-6">
			{@render children()}
		</main>
	</div>
</div>

<!-- {@render children()} -->
