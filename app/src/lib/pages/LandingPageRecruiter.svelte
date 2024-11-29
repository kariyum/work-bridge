<script lang="ts">
	import Project from '$lib/components/Project.svelte';
	import type { ProjectGET } from '$lib/types/project';
	let { projects }: { projects: ProjectGET[] } = $props();
</script>

<div class="outer-container">
	<div class="container">
		<h1>My Projects</h1>
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
	}
	.container {
		margin: 1rem 0rem;
	}
</style>
