<script lang="ts">
	import type { ProjectGET } from "$lib/types/project";

	let { project }: { project: ProjectGET } = $props();

	async function handleEasyApply() {
		const response = await fetch('/api/proposals', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({
				project_id: project.id,
			}),
		});
		if (response.ok) {
			console.log("Applied");
		} else {
			console.log("Failed to apply", response.status);
		}
	}

	function handleApply() {
		console.log('Apply');
	}
</script>

<div class="outer-container">
	<div class="container">
		<div class="user">
			<div class="avatar"></div>

			<h2>
				{project.user_id}
			</h2>
		</div>
		<h1>
			{project.title}
		</h1>
		<div>
			{@html project.content}
		</div>
		<div class="actions">
			<button type="button" onclick={handleEasyApply}>Easy apply</button>
			<button type="button" onclick={handleApply}>Apply</button>
			<button type="button">Not Interesed</button>
			<button type="button">Report</button>
		</div>
	</div>
</div>

<style>
	.container {
		border: 1px solid rgb(172, 172, 172);
		border-radius: 5px;
		margin: auto;
		padding: 1rem;
		max-width: 1280px;
	}

	.outer-container {
		margin: 1rem;
	}

	.avatar {
		display: inline-block;
		width: 3rem;
		height: 3rem;
		background-color: rgb(233, 233, 233);
		border: 1px solid rgb(197, 197, 197);
		border-radius: 50%;
	}

	.user {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-left: auto;
		width: fit-content;
	}
</style>
