<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import type { ProjectGET } from '$lib/types/project';
	import { formatDate } from '$lib/utils';
	let { project }: { project: ProjectGET } = $props();

    async function deleteProject() {
		const response = await fetch(`/api/projects/${project.id}`, {
			method: 'DELETE'
		});
		if (response.ok) {
			await invalidate('/api/projects');
		}
	}
</script>

<div class="outer-container">
	<div class="project">
		<div class="title">{project.title}</div>
		<div class="date">Created at {formatDate(project.created_at)}</div>
        <div class="price">Offering {project.budget} {project.currency_code}</div>
		<div class="content">
			{@html project.content}
		</div>
	</div>
    <div class="actions">
        <button onclick={deleteProject}>Delete</button>
        <button onclick={async () => await goto(`/project/${project.id}`)}>Edit</button>
        <!-- <button>Post Job</button> -->
    </div>
</div>

<style>
    .outer-container {
        padding: 1rem;
        /* cursor: pointer; */
        border: 1px solid #ddd;
        border-radius: 5px;
    }

    .content {
        margin-top: 1rem;
    }

    .title {
        font-size: x-large;
        font-weight: 500;
    }
    
    .actions {
        margin-left: auto;
        width: fit-content;
    }
</style>
