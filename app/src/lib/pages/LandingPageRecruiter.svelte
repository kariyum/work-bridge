<script lang="ts">
	import Project from '$lib/client/components/Project.svelte';
	import type { ProjectGET } from '$lib/types/project';
	let { projects }: { projects: ProjectGET[] } = $props();
</script>

<div class="outer-container">
	<h3>My Projects</h3>
	<div class="container">
		{#if projects.length === 0}
			<p>No projects found, please create one!</p>
		{:else}
			{#each projects as project}
				<Project {project} />
			{/each}
		{/if}
	</div>
</div>

<style>
	.outer-container {
		margin: auto;
		color: var(--dark-text);
	}
	.container {
		margin-top: 1rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
</style>
