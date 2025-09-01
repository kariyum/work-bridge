<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { TaskGET } from '$lib/types/task.js';
	import { snakeToCapital } from '$lib/utils.js';
	import { SquarePen } from 'lucide-svelte';
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
				pending: filterProposalsOnStatus(proposals, 'pending').length
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
	{@const sortedProposals = proposals.sort(
		(a, b) => a.created_at.getTime() - b.created_at.getTime()
	)}
	{#each sortedProposals as proposal}
		<div class="proposal">
			<div class="left">
				<div>{proposal.user_id}</div>
				<div>
					<span>Applied On:</span>
					<span>{task.created_at.toDateString()}</span>
				</div>
				<div>
					{#if proposal.budget}
						<span>Requesting: </span>
						<span>{proposal.budget}</span>
					{:else}
						<span>Budget not specified</span>
					{/if}
				</div>
			</div>
			<div class="right">
				<div class="status" data-type={proposal.status}>
					{snakeToCapital(proposal.status)}
				</div>
				<div>
					<!-- <a href="/messages?user_id={proposal.user_id}">Open discussion</a> -->
					<button
						class="muted-btn"
						onclick={() => patchProposalStatus(task.project_id, task.id, proposal.id, 'decline')}
						>Not Interested</button
					>
					<button
						onclick={() => patchProposalStatus(task.project_id, task.id, proposal.id, 'accept')}
						>Accept</button
					>
				</div>
			</div>
		</div>
	{/each}
{/snippet}

<div class="container">
	{#if task}
		<div class="task">
			<div class="row">
				<h2>{task.title}</h2>
				<div class="status" data-type={task.status}>
					{snakeToCapital(task.status)}
				</div>
			</div>
			<div class="body">
				<div class="column">
					<div class="card padding">
						<h2>Task Content</h2>
						{@html task.content}
					</div>
					<div class="card padding">
						<h2>Details</h2>
						<div class="details">
							<div class="detail">
								<span>Assignee</span>
								<span>{task.assignee_id}</span>
							</div>
							<div class="detail">
								<span>Deadline</span>
								<span>{task.deadline.toDateString()}</span>
							</div>
							<div class="detail">
								<span>Budget</span>
								<span>{task.budget}</span>
							</div>
							<div class="detail">
								<span>Required Skills</span>
								<div class="flex-row">
									{#if task.skills.length === 0}
										<div>No skills required.</div>
									{:else}
										{#each task.skills as skill}
											<div class="skill">{skill}</div>
										{/each}
									{/if}
								</div>
							</div>
						</div>
					</div>
				</div>
				<div>
					<div class="card">
						<h2 style="padding: 1rem; padding-bottom:0;">Task Actions</h2>
						<div class="actions">
							<button class="row">
								<div class="icon">
									<SquarePen />
								</div>
								<div class="btn-text">
									<div>Edit Task</div>
									<div>Change task details or content</div>
								</div>
							</button>
							<!-- <button>Close Task</button> -->
						</div>
					</div>
				</div>
			</div>
			<div class="applications">
				<h2>Applications ({filteredProposals?.length || 0})</h2>
				<div class="app-actions">
					<button data-active={filterStatus == undefined} onclick={() => (filterStatus = undefined)}
						>All</button
					>
					<button data-active={filterStatus == 'pending'} onclick={() => (filterStatus = 'pending')}
						>{proposalsCount?.pending ?? ''} Pending</button
					>
					<button
						data-active={filterStatus == 'accepted'}
						onclick={() => (filterStatus = 'accepted')}
						>{proposalsCount?.accepted ?? ''} Accepted</button
					>
					<button
						data-active={filterStatus == 'declined'}
						onclick={() => (filterStatus = 'declined')}
						>{proposalsCount?.declined ?? ''} Declined</button
					>
				</div>
				<div class="proposals">
					{#if filteredProposals && filteredProposals.length !== 0}
						{@render proposalsSnippet(filteredProposals, task)}
					{:else if filteredProposals && filterStatus}
						<div>0 {filterStatus} applications</div>
					{:else}
						<div>No applications yet!</div>
					{/if}
				</div>
			</div>
		</div>
	{:else}
		<div>Task Not Found !?</div>
	{/if}
</div>

<style>
	.flex-row {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}
	.btn-text {
		text-align: left;

		> div:last-child {
			color: var(--sub-title);
		}
	}
	.icon {
		line-height: 0;
	}
	.left {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;

		> div:not(:first-child) {
			color: var(--sub-title);
		}
	}
	.right {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
	.details {
		display: grid;
		grid-template-columns: 1fr 1fr;
		row-gap: 1rem;
		column-gap: 1rem;
	}
	.detail {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;

		> span:first-child {
			color: var(--sub-title);
		}
	}
	.column {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}
	.task {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.row {
		display: flex;
		gap: 1rem;
	}
	.body {
		display: grid;
		grid-template-columns: 5fr 2fr;
		column-gap: 2rem;
	}
	.card {
		border-radius: 15px;
		background-color: var(--card-bg);
		border: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.padding {
		padding: 2rem;
	}
	.applications {
		margin: 1rem 0;
		> * {
			margin-bottom: 1rem;
		}

		.app-actions {
			display: flex;
			padding-bottom: 0.5rem;
			gap: 0.2rem;
			border-bottom: 1px solid var(--border);

			button {
				position: relative;
				background-color: transparent;
				border: none;
				padding: 0;
				padding: 0.5rem 0.5rem 0.5rem 0.5rem;
			}

			button[data-active='true'] {
				background-color: var(--selected-color);
			}
			button[data-active='true']::after {
				position: absolute;
				content: '';
				background-color: var(--blue);
				width: 100%;
				height: 1px;
				bottom: -8px;
				border-radius: 15px;
				left: 0;
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
		> button:first-child {
			border-bottom: 0;
		}
		> button:last-child {
			border-top: 0;
			border-radius: 0 0 15px 15px;
		}
		> button {
			width: 100%;
			border-radius: 0;
		}
	}

	.proposal {
		width: 100%;
	}

	.proposals {
		display: flex;
		flex-direction: column;
		> .proposal:first-child {
			border-top-left-radius: 15px;
			border-top-right-radius: 15px;
		}
		> .proposal:last-child {
			border-bottom-left-radius: 15px;
			border-bottom-right-radius: 15px;
		}
		> .proposal {
			padding: 1.5rem;
			background-color: var(--card-bg);
			border: 1px solid var(--border);
			display: flex;
			flex-direction: row;
			justify-content: space-between;
		}
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
		max-width: var(--page-width);
		margin: 1rem auto;
	}
</style>
