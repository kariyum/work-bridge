<script lang="ts">
	import Project from '$lib/components/project/client/Project.svelte';
	import type { ProjectGET } from '$lib/types/project';
	let { projects }: { projects: ProjectGET[] } = $props();
</script>

<div class="outer-container">
	<h3>My Projects</h3>
	<div>
		{#if projects.length === 0}
			<p>No projects found, please create one!</p>
		{:else}
			<div class="container">
				{#each projects as project}
					<Project {project} />
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.outer-container {
		margin: auto;
	}
	.container {
		margin-top: 1rem;
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
	}
</style>
