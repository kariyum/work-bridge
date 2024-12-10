<script lang="ts">
	import type { ProjectGET } from '$lib/types/project';

	let { project }: { project: ProjectGET } = $props();
	async function handleEasyApply() {
		const response = await fetch('/api/proposals', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				project_id: project.id
			})
		});
		if (response.ok) {
			console.log('Applied');
		} else {
			console.log('Failed to apply', response.status);
		}
	}

	function handleApply() {
		console.log('Apply');
	}
</script>

<div class="outer-container">
	<div class="user">
		<div class="avatar"></div>
		<h2>
			{project.user_id}
		</h2>
	</div>
	<div class="container">
		<h2>{project.title}</h2>
		<div>
			Created at: {project.created_at.getDate()}/{project.created_at.getMonth()}/{project.created_at.getFullYear()}
		</div>
		<div>Description:</div>
		<div>
			Deadline: {project.deadline.getDate()}/{project.deadline.getMonth()}/{project.deadline.getFullYear()}
		</div>
		<div>
			Offering: {`${project.budget} ${project.currency_code}`}
			{project.currency_code}
		</div>
		<div class="content">
			Description: 
			{@html project.content}
		</div>
	</div>
	<div class="actions">
		<button type="button" onclick={handleEasyApply}>Easy apply</button>
		<button type="button" onclick={handleApply}>Apply</button>
		<!-- <button type="button">Not Interesed</button> -->
		<!-- <button type="button">Report</button> -->
	</div>
</div>

<style>
	h1 {
		padding: 1rem;
	}

	.container {
		padding: 0.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}


	.outer-container {
		margin-top: 1rem;
		border: 1px solid rgb(204, 204, 204);
		border-radius: 5px;
	}

	.avatar {
		display: inline-block;
		width: 2.5rem;
		height: 2.5rem;
		background-color: rgb(233, 233, 233);
		border: 1px solid rgb(197, 197, 197);
		border-radius: 50%;
		background-image: linear-gradient(red, rgb(0, 110, 255));
	}

	.user {
		padding: 0.5rem;
		border-bottom: 1px solid #eee;
		display: flex;
		gap: 1rem;
		border-radius: 4px 4px 0px 0px;
		background-color: #fbfbfb;
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		background-color: #fbfbfb;
		display: flex;
		gap: 1rem;
		/* margin-left: auto; */
		/* width: fit-content; */
		padding: 0.5rem;
		border-top: 1px solid #eee;
		border-radius: 0px 0px 4px 4px;
	}
</style>
