<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import Task from './Task.svelte';

	let { project }: { project: ProjectObject } = $props();

	async function deleteProject() {
		const response = await fetch(`/api/projects/${project.id}`, {
			method: 'DELETE'
		});
		if (response.ok) {
			await invalidate('/api/projects');
		}
	}
</script>

<div class="top-container">
	<div class="title-container">
		<h2>{project.title}</h2>
	</div>
	<div class="container">
		<div>
			Created at: {project.created_at.getDate()}/{project.created_at.getMonth()}/{project.created_at.getFullYear()}
		</div>
		<div>
			Offering: {`${project.budget} ${project.currency_code}`}
		</div>
		<div>Description:</div>
		<div class="project-content">
			{@html project.content}
		</div>
		<div>
			Deadline: {project.deadline.getDate()}/{project.deadline.getMonth()}/{project.deadline.getFullYear()}
		</div>
		<Task taskObject={undefined}></Task>
	</div>
	<div class="actions-container">
		<div class="actions">
			<button onclick={async () => await goto(`/project/${project.id}`)}>Edit</button>
			<button onclick={deleteProject}>Delete</button>
		</div>
	</div>
</div>

<style>
	.title-container {
		background-color: #fafafa;
		padding: 0.5rem 1rem;
		border-bottom: 1px solid #eee;
	}
	h2 {
		background-color: inherit;
	}
	.top-container {
		border: 1px solid rgb(204, 204, 204);
		border-radius: 3px;
		margin: 1rem auto 1rem auto;
	}
	.actions-container {
		border-top: 1px solid #eee;
		padding: 0.5rem 0.5rem;
		border-radius: 0px 0px 5px 5px;
		background-color: #fbfbfb;
	}
	.container {
		padding: 1rem;
		border-radius: 5px;
		white-space: pre-wrap;
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-left: auto;
		width: fit-content;
		background-color: inherit;
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

	.project-content {
		padding: 0.5rem;
		border: 1px solid #eee;
		border-radius: 5px;
	}
</style>
