<script lang="ts">
	import { pushState } from '$app/navigation';
	import { Plus, Trash } from 'lucide-svelte';

	import { page } from '$app/state';
	import { TaskClass } from '$lib/components/task/states.svelte';
	import TaskForm from './TaskForm.svelte';

	let {
		projectId,
		tasks = $bindable()
	}: {
		projectId?: number;
		tasks: TaskClass[];
	} = $props();

	function addNewTask() {
		selectedTask = -1;
		pushState('', {
			projectEditMode: true,
			showTaskPopup: true,
			profileEditMode: false
		});
	}

	function onSubmit(task: TaskClass) {
		if (task.id || selectedTask !== -1) {
			const taksIndex = tasks.findIndex((t) => t.id === task.id);
			tasks[taksIndex] = task;
		} else {
			tasks.push(task);
		}
		history.back();
	}

	function removeTask(index: number) {
		tasks.splice(index, 1);
	}

	let selectedTask = $state(0);
</script>

<div class="headline">
	<h2>Tasks</h2>
	<button onclick={addNewTask} class="add-task"
		><Plus size="18" />
		<p>Add Task</p></button
	>
</div>
<div class="tasks-container">
	{#if tasks.length === 0}
		<p>Add tasks by clicking on the plus (+) button.</p>
	{:else}
		{#each tasks as task, i}
			<div class="task-container">
				<button
					class="single-task"
					onclick={() => {
						selectedTask = i;
						pushState('', {
							projectEditMode: true,
							showTaskPopup: true,
							profileEditMode: false
						});
					}}
				>
					{task.title}
				</button>
				<button aria-label="delete" onclick={() => removeTask(i)}><Trash size="18" /></button>
			</div>
		{/each}
	{/if}
</div>

{#if page.state.showTaskPopup}
	<TaskForm taskInput={tasks[selectedTask]} onSubmit={(task) => onSubmit(task)} />
{/if}

<style>
	.add-task {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		cursor: pointer;
		line-height: 0;
	}
	.tasks-container {
		display: flex;
		flex-wrap: nowrap;
		flex-direction: column;
		margin-top: 0.5rem;
		gap: 0.5rem;
	}
	.headline {
		padding: 0.5rem 0rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.single-task {
		display: inline;
		width: 100%;
		text-align: left;
		background-color: transparent;
		border-radius: 0;
		border: none;
		&:hover {
			background-color: var(--hover-color);
		}
	}
	.task-container {
		display: flex;
		flex-direction: row nowrap;
		border: 2px solid var(--border);
		border-radius: 5px;
		overflow: hidden;

		button[aria-label='delete'] {
			border: none;
			border-radius: 0;
			display: inline;
			margin: 0;
			padding: 0.5rem;
			line-height: 0;
		}

		button {
			align-self: stretch;
		}
	}
</style>
