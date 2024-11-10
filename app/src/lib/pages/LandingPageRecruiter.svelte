<script lang="ts">
	import Project from '$lib/components/Project.svelte';
	let { projects }: { projects: ProjectObject[] } = $props();
</script>

<div class="outer-container">
	<div class="container">
		<h1>My Projects</h1>
		{#each projects as project}
			<Project project={project} />
		{/each}
	</div>
</div>

<style>
	.outer-container {
		max-width: 1200px;
		margin: auto;
	}
    .container {
        margin: 1rem;
    }
</style>
