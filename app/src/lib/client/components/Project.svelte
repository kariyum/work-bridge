<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import type { ProjectGET } from '$lib/types/project';
	import { formDateSentence } from '$lib/utils';
	let { project }: { project: ProjectGET } = $props();

	async function deleteProject() {
		const response = await fetch(`/api/projects/${project.id}`, {
			method: 'DELETE'
		});
		if (response.ok) {
			await invalidate('/api/projects');
		}
	}

	function getRandomBgClass(id: number) {
		const classes = ['blue-bg', 'pink-bg', 'violet-bg', 'beige-bg'];
		return classes[id % classes.length];
	}
</script>

<div class="outer-container">
	<div
		class="project {getRandomBgClass(
			project.content.length + project.title.length + (project.id ?? 0)
		)}"
	>
		<div class="project-top">
			<div class="date">{formDateSentence(project.created_at)}</div>
		</div>
		<div class="title">{project.title}</div>
		<div class="content">
			{@html project.content}
		</div>
	</div>
	<div class="actions">
		<div class="price">{project.budget} {project.currency_code}</div>
		<div class="button">
			<button onclick={deleteProject}>Delete</button>
			<button onclick={async () => await goto(`/project/${project.id}`)}>Edit</button>
		</div>
		<!-- <button>Post Job</button> -->
	</div>
</div>

<style>
	:global(p) {
		display: block;
		min-height: 1rem;
	}
	.outer-container {
		padding: 0.5rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		/* cursor: pointer; */
		border: 2px solid var(--border);
		border-radius: 10px;
		min-height: 20rem;
	}

	.content {
		margin-top: 1rem;
	}

	.title {
		font-size: x-large;
		font-weight: 500;
	}

	.price {
		font-weight: 500;
		font-size: large;
		margin-left: 0.5rem;
	}

	.actions {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.date {
		border: 2px solid var(--border);
		background-color: var(--background-color);
		width: fit-content;
		padding: 0.3rem 0.7rem;
		border-radius: 20px;
	}

	.project {
		border-radius: 10px;
		padding: 1rem;
		height: 100%;
        
	}
</style>
