<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { TaskGET } from '$lib/types/task.js';
	import { fetchIntoResult, snakeToCapital } from '$lib/utils.js';
	import type { ProposalGET } from './+page.js';

	let { data } = $props();
	let task = $derived(data.task);
	let proposals: ProposalGET[] | undefined = $derived(data.proposals);

	async function patchProposalStatus(
		projectId: number,
		taksId: number,
		proposalId: number,
		action: string
	) {
		const payload = {
			action
		};
		const response = await fetch(`/api/proposals/${proposalId}/status`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});
		if (response.ok) {
			await invalidate(`/api/projects/${projectId}/task/${taksId}/proposals`);
		} else {
			// todo show error banner
		}
	}
</script>

<div class="container">
	{#if task && proposals}
		<div class="task">
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
			<div class="task-content">
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
		</div>
		<div>
			<h1>Applications</h1>
			<div class="proposals">
				{#if proposals.length !== 0}
					{#each proposals as proposal}
						<div class="proposal">
							<div class="status" data-type={proposal.status}>
								{snakeToCapital(proposal.status)}
							</div>
							<div>
								{proposal.user_id}
							</div>
							<div>
								{#if proposal.budget}
									Requesting: {proposal.budget}
								{:else}
									Budget not specified
								{/if}
							</div>

							<div class="actions">
								<a href="/messages?user_id={proposal.user_id}">Open discussion</a>
								<button
									class="muted-btn"
									onclick={() =>
										patchProposalStatus(task.project_id, task.id, proposal.id, 'reject')}
									>Not Interested</button
								>
								<button
									onclick={() =>
										patchProposalStatus(task.project_id, task.id, proposal.id, 'accept')}
									>Accept</button
								>
							</div>
						</div>
					{/each}
				{:else}
					<div>No applications yet!</div>
				{/if}
			</div>
		</div>
	{:else}
		<div>Task Not Found !?</div>
	{/if}
</div>

<style>
	.status {
		margin-bottom: 0.3rem;
	}

	.muted-btn {
		background-color: transparent;
	}

	.actions {
		margin-left: auto;
		width: fit-content;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.proposal {
		width: 100%;
	}

	.proposals {
		margin-top: 1rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.task-content {
		margin: 1rem 0;
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
		max-width: 1100px;
		margin: auto;
	}
</style>
