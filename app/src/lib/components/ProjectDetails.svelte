<script lang="ts">
	import type { ProjectGET } from '$lib/types/project';
	interface props {
		projectIn: ProjectGET;
		role: string;
		userId: string;
		onedit: () => void;
	}
	let { projectIn, role, onedit, userId }: props = $props();
</script>

<div class="container">
	<div class="sub-container">
		<div class="header">
			<h1>#{projectIn.id} {projectIn.title}</h1>
			{#if projectIn.user_id === userId}
				<button class="edit-btn" onclick={onedit}>Edit</button>
			{/if}
		</div>
		<div>
			<span style="font-weight: 500;">Client's email:</span>
			<span>{projectIn.user_id}</span>
		</div>
		<p class="content">
			{#if projectIn.content.length === 0}
				<span>No content for this project</span>
			{:else}
				{@html projectIn.content}
			{/if}
		</p>

		<hr />
		<h2>Tasks</h2>
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
						<div class="task-content">
							{#if task.content.length === 0}
								<div>No content for this task</div>
							{:else}
								{@html task.content}
							{/if}
						</div>
						<h4>Required Skills</h4>
						<div class="skills">
							{#if task.skills.length === 0}
								<div>No skills required.</div>
							{:else}
								{#each task.skills as skill}
									<div class="skill">{skill}</div>
								{/each}
							{/if}
						</div>
						<div>
							<span style="font-weight: 500;">Assigned to: </span>
							<span>{task.assignee_id}</span>
						</div>
						{#if role === 'freelancer'}
							<button class="apply-btn">Submit Application</button>
						{/if}
					</div>
				{/each}
			</div>
		{:else}
			<div>No tasks are available for this project yet!</div>
		{/if}
	</div>
</div>

<style>
	.header {
		display: flex;
		justify-content: space-between;
	}
	.edit-btn {
		background-color: var(--blue);
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
		gap: 1rem;
	}

	.task {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		border: 2px solid var(--border);
		padding: 0.5rem 0.5rem;
		border-radius: 5px;
	}

	.skills {
		display: flex;
		gap: 0.5rem;
	}

	.skill {
		background-color: var(--btn-bg);
		padding: 0.3rem 0.8rem;
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
