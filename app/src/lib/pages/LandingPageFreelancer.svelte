<script lang="ts">
	import Project from '$lib/components/project/freelancer/Project.svelte';
	import type { ProjectGET } from '$lib/types/project';
	let { projects }: { projects: ProjectGET[] } = $props();
</script>

<div class="outer-container">
	<h3>Available Projects</h3>
	<div>
		{#if projects.length === 0}
			<p>Sorry for the inconvenience, no projects were found.</p>
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
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}
</style>
