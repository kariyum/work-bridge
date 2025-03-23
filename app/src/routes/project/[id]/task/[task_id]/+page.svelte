<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { TaskGET } from '$lib/types/task.js';
	import { snakeToCapital } from '$lib/utils.js';
	import type { ProposalGET } from './+page.js';

	let { data } = $props();
	let task = $derived(data.task);
	let proposals: ProposalGET[] | undefined = $derived(data.proposals);
	const filterProposalsOnStatus = (proposals: ProposalGET[], status: string) =>
		proposals?.filter((prop) => prop.status.toLowerCase() === status);

	let proposalsCount = $derived.by(() => {
		if (proposals) {
			return {
				accepted: filterProposalsOnStatus(proposals, 'accepted').length,
				declined: filterProposalsOnStatus(proposals, 'declined').length,
				pending: filterProposalsOnStatus(proposals, 'pending').length,
			};
		}
	});

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

	let filterStatus: string | undefined = $state(undefined); // enum: all, pending, rejected, accepted
	let filteredProposals = $derived.by(() => {
		if (filterStatus && proposals) {
			return filterProposalsOnStatus(proposals, filterStatus);
		} else return proposals;
	});
</script>

{#snippet proposalsSnippet(proposals: ProposalGET[], task: TaskGET)}
	{@const sortedProposals = proposals.sort((a, b) => a.created_at.getTime() - b.created_at.getTime())}
	{#each sortedProposals as proposal}
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
					onclick={() => patchProposalStatus(task.project_id, task.id, proposal.id, 'decline')}
					>Not Interested</button
				>
				<button onclick={() => patchProposalStatus(task.project_id, task.id, proposal.id, 'accept')}
					>Accept</button
				>
			</div>
		</div>
	{/each}
{/snippet}

<div class="container">
	{#if task && filteredProposals}
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
		</div>
		<div class="applications">
			<h1>Applications</h1>
			<div class="app-actions">
				<button data-active={filterStatus == undefined} onclick={() => (filterStatus = undefined)}
					>All</button
				>
				<button data-active={filterStatus == 'pending'} onclick={() => (filterStatus = 'pending')}
					>{proposalsCount?.pending ?? ''} Pending</button
				>
				<button data-active={filterStatus == 'accepted'} onclick={() => (filterStatus = 'accepted')}
					>{proposalsCount?.accepted ?? ''} Accepted</button
				>
				<button data-active={filterStatus == 'declined'} onclick={() => (filterStatus = 'declined')}
					>{proposalsCount?.declined ?? ''} Declined</button
				>
			</div>
			<div class="proposals">
				{#if filteredProposals.length !== 0}
					{@render proposalsSnippet(filteredProposals, task)}
				{:else if filterStatus}
					<div>0 {filterStatus} applications</div>
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
	.applications {
		h1 {
			margin-bottom: 1rem;
		}

		.app-actions {
			display: flex;
			gap: 0.2rem;

			button {
				background-color: transparent;
				border: none;
				padding: 0;
				padding: 0.1rem 0.6rem 0.1rem 0.6rem;
			}

			button[data-active='true'] {
				background-color: var(--selected-color);
			}

			button[data-active='false'] {
				color: rgba(var(--font-color), 0.1);
			}
		}
	}
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
