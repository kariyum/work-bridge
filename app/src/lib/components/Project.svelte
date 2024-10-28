<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import Task from './Task.svelte';

	let { project }: { project: Project } = $props();

	async function deleteProject() {
		const response = await fetch(`/api/projects/${project.id}`, {
			method: 'DELETE'
		});
		if (response.ok) {
			await invalidate('/api/projects');
		}
	}
</script>

<div class="container">
	<h1>{project.title} for {project.budget} {project.currency_code}</h1>
	{@html project.content}
	<div>
		{project.deadline}
	</div>
	<Task>
		
	</Task>
	<div class="actions">
		<button onclick={async () => await goto(`/project/${project.id}`)}>Edit</button>
		<button onclick={deleteProject}>Delete</button>
	</div>
</div>

<style>
	.container {
		padding: 1rem;
		border: 1px solid rgb(204, 204, 204);
		border-radius: 5px;
		margin: 1rem auto 1rem auto;
		max-width: 60%;
		white-space: pre-wrap;
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-top: 1rem;
		margin-left: auto;
		width: fit-content;
	}
	:global(ol) {
		margin-left: 2rem;
	}

	:global(ul) {
		margin-left: 2rem;
	}

	:global(p) {
		display: block;
		min-height: 1rem;
	}
	
</style>
