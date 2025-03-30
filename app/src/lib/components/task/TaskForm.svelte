<script lang="ts">
	import RichTextEditor from '$lib/components/texteditor/RichTextEditor.svelte';
	import Skills from '$lib/components/skills/Skills.svelte';
	import { TaskClass } from '$lib/components/task/states.svelte';
	interface props {
		taskInput?: TaskClass;
		onSubmit: (x: TaskClass) => void;
	}
	let { taskInput, onSubmit }: props = $props();

	function cancel() {
		history.back();
	}

	let taskClass = $derived(taskInput?.copy() ?? new TaskClass());
	const isEditingMode = taskInput !== undefined;
</script>

<div class="blur">
	<div class="popover">
		<div class="create-task">
			{#if isEditingMode}
				<h2>Edit task</h2>
			{:else}
				<h2>Add task</h2>
			{/if}
			<form class="input-container" onsubmit={(event) => event.preventDefault()}>
				<input type="text" placeholder="Title" bind:value={taskClass.title} />
				<RichTextEditor bind:x={taskClass.content}></RichTextEditor>
				<input type="text" placeholder="Assignee" bind:value={taskClass.assignee_id} />
				<!-- <input type="text" placeholder="Skills" bind:value={taskClass.skills} /> -->
				<div class="skills-input">
					<Skills
						skillsIn={taskClass.skills}
						addSkill={(skill) => taskClass.addSkill(skill)}
						removeSkillAtIndex={(index) => taskClass.removeSkillIndex(index)}
					></Skills>
				</div>
				<!-- <input type="text" placeholder="Status" bind:value={taskClass.status} /> -->
				<select name="status" id="status" bind:value={taskClass.status}>
					<option value="todo">Todo</option>
					<option value="inprogress">In Progress</option>
					<option value="done">Done</option>
				</select>
				<input type="text" placeholder="Budget" bind:value={taskClass.budget} />
				<input type="date" placeholder="Deadline" bind:value={taskClass.deadline} />
				<!-- <input type="text" placeholder="Estimated Efforts" /> -->
				<div class="act-task">
					<button class="cancel-btn" onclick={cancel}>Cancel</button>
					<button onclick={() => onSubmit(taskClass)}>
						{#if isEditingMode}
							Update
						{:else}
							Add
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
</div>

<style>
	select {
		padding: 0.5rem;
		border-radius: 5px;
		border: 2px solid var(--border);
	}
	.act-task {
		margin-left: auto;
	}
	.input-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
	.create-task {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.popover {
		position: absolute;
		top: 10%;
		left: 50%;
		width: 100%;
		max-width: 1000px;
		margin: auto 1rem auto 1rem;
		transform: translate(-50%, 0);
		padding: 1rem;
		border: 2px solid var(--border);
		border-radius: 5px;
		background-color: var(--background-color);
	}

	.blur {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		backdrop-filter: blur(2px);
		background-color: rgba(0, 0, 0, 0.5);
	}

	.skills-input {
		width: max-width;
	}
</style>
