<script lang="ts">
	import { goto } from '$app/navigation';
	import RichTextEditor from '$lib/components/RichTextEditor.svelte';
	import Skills from '$lib/components/Skills.svelte';
	import { TaskClass } from '$lib/states.svelte';
	import { untrack } from 'svelte';
	let { data } = $props();
	const tasksGlobalState = $derived(data.tasksGlobalState);

	$effect.pre(() => {
		// effect needed when the page is refreshed
		data.selectedIndex;
		untrack(() => {
			if (
				data.selectedIndex != undefined &&
				data.selectedIndex != null &&
				tasksGlobalState.selected === -1
			) {
				if (0 <= data.selectedIndex && data.selectedIndex < tasksGlobalState.tasks.length) {
					tasksGlobalState.selectTask(data.selectedIndex);
				}
			}
		});
	});

	function initTaskClass() {
		let result;
		if (tasksGlobalState.selected != -1) {
			result = tasksGlobalState.tasks[tasksGlobalState.selected].copy();
		} else {
			result = new TaskClass();
		}
		return result;
	}

	function updateTaskClass() {
		if (tasksGlobalState.selected != -1) {
			tasksGlobalState.tasks[tasksGlobalState.selected].assignFrom(taskClass);
		} else {
			tasksGlobalState.tasks.push(taskClass);
		}
	}

	let taskClass = $state(initTaskClass());

	function add(event: Event) {
		event.preventDefault();
		updateTaskClass();
		tasksGlobalState.selected = -1;
		history.back();
	}

	async function cancel() {
		tasksGlobalState.selected = -1;
		const url = window.location.pathname.split('/').slice(0, -1).join('/');
		await goto(url);
	}

	const isEditingMode = $derived(tasksGlobalState.selected != -1);
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
				<RichTextEditor contentIn={(() => taskClass.content)()} bind:x={taskClass.content}></RichTextEditor>
				<input type="text" placeholder="Assignee" bind:value={taskClass.assignee_id} />
				<!-- <input type="text" placeholder="Skills" bind:value={taskClass.skills} /> -->
				<div class="skills-input">
					<Skills skillsIn={taskClass.skills} addSkill={(skill) => taskClass.addSkill(skill)} removeSkillAtIndex={(index) => taskClass.removeSkillIndex(index)}></Skills>
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
					<button onclick={cancel}>Cancel</button>
					<button onclick={add}>Add</button>
				</div>
			</form>
		</div>
	</div>
</div>

<style>
	select {
		padding: 0.5rem;
		border-radius: 5px;
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
		top: 50%;
		left: 50%;
		width: 50%;
		max-width: 1000px;
		transform: translate(-50%, -50%);
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
		background-color: rgba(0, 0, 0, 0.1);
	}

	.skills-input {
		width: max-width;
	}
</style>
