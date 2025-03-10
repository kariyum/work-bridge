<script lang="ts">
	let { data } = $props();
	let task = $derived(data.task);
	let proposals = $derived(data.proposals)
</script>

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
		<h1>Proposals</h1>
		{#each proposals as proposal}
			<div>
				{proposal.user_id}
			</div>
			<div>
				{proposal.budget}
			</div>
		{/each}
	</div>
{:else}
	<div>Task Not Found !?</div>
{/if}

<style>
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
</style>
