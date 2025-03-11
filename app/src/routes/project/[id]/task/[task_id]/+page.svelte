<script lang="ts">
	let { data } = $props();
	let task = $derived(data.task);
	let proposals = $derived(data.proposals);
</script>

<div class="container">
	{#if task && proposals}
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
				{#each proposals as proposal}
					<div class="proposal">
						<div>
							{proposal.user_id}
						</div>
						<div>
							{#if proposal.budget}
								Requesting: {proposal.budget}
							{:else}
								Budget Not Specified
							{/if}
						</div>
						<div>
							{proposal.status}
						</div>
						<a href="/messages">Open discussion</a>
						<button>Accept</button>
						<button>Not Interested</button>
					</div>
				{/each}
			</div>
		</div>
	{:else}
		<div>Task Not Found !?</div>
	{/if}
</div>

<style>
	.proposals {
		margin-top: 1rem;
		display: flex;
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
