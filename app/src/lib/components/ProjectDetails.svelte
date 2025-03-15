<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { page } from '$app/state';
	import type { Tab } from '$lib/types';
	import type { ProjectGET } from '$lib/types/project';
	import { formatDate } from '$lib/utils';
	import Tabs from './Tabs.svelte';
	interface props {
		projectIn: ProjectGET;
		role: string;
		userId: string;
		onEdit: () => void;
	}
	let { projectIn, role, userId, onEdit }: props = $props();

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
			console.log('OK');
			await invalidate(`/api/projects/${projectIn.id}`);
		} else {
			alert('Failed for submit to task!');
		}
	}
</script>

{#snippet tasksSnippet()}
	{#if projectIn.tasks?.length !== 0}
		<div class="tasks-container">
			{#each projectIn.tasks?.sort((a, b) => a.id - b.id) ?? [] as task}
				<div class="task">
					<h3>
						#{task.id}
						{task.title}
					</h3>
					<div class="status" data-type={task.status}>
						{task.status}
					</div>
					<div>
						Budget: {task.budget}
					</div>
					<div>
						Assigned to: {task.assignee_id}
					</div>
					<div class="task-content rich-content">
						Content:
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

					{#if role === 'freelancer'}
						{#if task.application_submitted}
							<button class="applied-btn" disabled>Application submitted</button>
						{:else}
							<button class="apply-btn" onclick={() => submitApplication(task.id)}
								>Submit Application</button
							>
						{/if}
					{:else if role === 'recruiter'}
						<div class="view-link">
							<a href={`/project/${projectIn.id}/task/${task.id}`}> View </a>
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
			{#if projectIn.user_id === userId}
				<button onclick={onEdit}>Edit</button>
			{/if}
		</div>
		<div>
			<span style="font-weight: 500;">Client's email:</span>
			<span>{projectIn.user_id}</span>
		</div>
		<p class="content rich-content">
			{#if projectIn.content.length === 0}
				<span>No content for this project</span>
			{:else}
				{@html projectIn.content}
			{/if}
		</p>
		<div>
			<span style="font-weight: 500;">Posted on: </span>
			{formatDate(projectIn.created_at)}
		</div>
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
	.applied-btn {
		background-color: rgb(65, 65, 65);
	}
	.header {
		display: flex;
		justify-content: space-between;
	}
	.edit-btn {
		padding: 0.3rem 1rem;
		text-decoration: none;
		color: inherit;
		line-height: 2rem;
		border-radius: 5px;
		background-color: var(--btn-bg);
	}
	.apply-btn {
		background-color: var(--blue);
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
	hr {
		border: none;
		border-top: 2px solid var(--border);
		margin: 1rem 0 1rem 0;
	}
	.emoji {
		margin: 0;
		padding: 0.3rem;
		background-color: transparent;
		border: none;
	}

	.actions-container {
		display: flex;
		justify-content: space-between;
	}

	.container {
		margin-top: 1rem;
	}

	.sub-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		max-width: 1100px;
		margin: auto;
	}

	.content {
		min-height: 5rem;
		margin-top: 0.5rem;
		border-radius: 5px;
	}

	.actions {
		display: flex;
		width: fit-content;
	}

	.left-chip {
		padding: 0.5rem 1rem 0.5rem 0.7rem;
		border-radius: 2rem 0 0 2rem;
	}

	.right-chip {
		padding: 0.5rem 1rem 0.5rem 0.7rem;
		border-radius: 0 2rem 2rem 0;
	}

	.vote {
		background-color: var(--btn-bg);
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin: 0;
		border: none;
		font-size: 1rem;
	}

	.vote:hover {
		background-color: var(--blue);
	}

	.vertical-line {
		background-color: var(--grey);
		width: 1px;
	}

	.comments {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.comment-input {
		width: 100%;
		display: flex;
		gap: 1rem;
		align-items: center;
		margin-top: 1rem;
	}

	.comment-input > input {
		width: 100%;
	}
</style>
