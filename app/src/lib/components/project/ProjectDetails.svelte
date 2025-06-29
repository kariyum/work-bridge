<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { User } from '$lib/types';
	import type { ProjectGET } from '$lib/types/project';
	import { formatDate, snakeToCapital } from '$lib/utils';
	interface props {
		projectIn: ProjectGET;
		user?: User;
		onEdit: () => void;
	}
	let { projectIn, user, onEdit }: props = $props();
	let userIsCreator = $derived(user?.email == projectIn.user_id);

	async function submitApplication(taskId: number) {
		const payload = {
			task_id: taskId
		};
		const response = await fetch('/api/proposals', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});
		if (response.ok) {
			await invalidate(`/api/projects/${projectIn.id}`);
		} else {
			alert('Failed to submit proposal!');
		}
	}
</script>

{#snippet tasksSnippet()}
	{#if projectIn.tasks?.length !== 0}
		<div class="tasks-container">
			{#each projectIn.tasks?.sort((a, b) => a.id - b.id) ?? [] as task}
				<div class="task" id={task.id.toString()}>
					<h3>
						#{task.id}
						{task.title}
					</h3>
					<div class="status" data-type={task.status}>
						{snakeToCapital(task.status)}
					</div>
					<div>
						Budget: {task.budget}
					</div>
					<div>
						Assigned to: {task.assignee_id}
					</div>
					<div class="task-content rich-content">
						Description:
						{#if task.content.length === 0}
							<div>No content for this task</div>
						{:else}
							{@html task.content}
						{/if}
					</div>
					<div class="skills">
						{#if task.skills.length === 0}
							<div>No skills required.</div>
						{:else}
							{#each task.skills as skill}
								<div class="skill">{skill}</div>
							{/each}
						{/if}
					</div>

					{#if user?.role === 'freelancer'}
						<button
							class="apply-btn"
							disabled={task.proposal_status != undefined}
							data-status={task.proposal_status}
							onclick={() => submitApplication(task.id)}
						>
							{#if task.proposal_status}
								Application {snakeToCapital(task.proposal_status)}
							{:else}
								Submit Application
							{/if}
						</button>
					{:else if user?.role === 'recruiter' && userIsCreator}
						<div class="view-link">
							<a href={`/project/${projectIn.id}/task/${task.id}`}>View</a>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{:else}
		<div>No tasks are available for this project yet!</div>
	{/if}
{/snippet}

<div class="container">
	<div class="sub-container">
		<div class="header">
			<h1>#{projectIn.id} {projectIn.title}</h1>
			{#if userIsCreator}
				<button onclick={onEdit}>Edit</button>
			{/if}
		</div>
		<div>
			<span style="font-weight: 500;">Client's email:</span>
			<span>{projectIn.user_id}</span>
		</div>
		<div>
			<span style="font-weight: 500;">Posted on: </span>
			{formatDate(projectIn.created_at)}
		</div>
		<div class="content rich-content">
			{#if projectIn.content.length === 0}
				<span>No content for this project</span>
			{:else}
				{@html projectIn.content}
			{/if}
		</div>
		<hr style="margin: 0.5rem 0; color: black;" />
		<h2>Tasks</h2>
		{@render tasksSnippet()}
	</div>
</div>

<style>
	.view-link {
		width: 100%;

		a {
			display: block;
			width: fit-content;
			margin-left: auto;
		}
	}

	.header {
		display: flex;
		justify-content: space-between;
	}

	.apply-btn {
		background-color: var(--blue);
	}

	.apply-btn[data-status='pending'] {
		background-color: var(--orange);
	}
	.apply-btn[data-status='declined'] {
		background-color: unset;
	}
	.apply-btn[data-status='approved'] {
		background-color: var(--green);
	}
	.apply-btn[data-status='cancelled'] {
		background-color: var(--grey);
	}

	.task-content {
		margin: 1rem 0;
	}

	.tasks-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.task {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		/* border: 2px solid var(--border); */
		padding: 0.5rem 0;
		border-radius: 5px;
	}

	.skills {
		display: flex;
		gap: 0.5rem;
	}

	.skill {
		background-color: var(--tag-bg);
		padding: 0.5rem 0.8rem;
		align-items: center;
		border-radius: 50px;
		width: fit-content;
		line-height: 1rem;
	}

	.container {
		margin-top: 1rem;
	}

	.sub-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		max-width: var(--page-width);
		margin: auto;
	}

	.content {
		margin: 0.5rem 0 0.5rem 0;
		border-radius: 5px;
	}
</style>
