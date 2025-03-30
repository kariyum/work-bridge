<script lang="ts">
	import { pushState } from '$app/navigation';
	import { Plus } from 'lucide-svelte';

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
			showTaskPopup: true
		});
	}

	function onSubmit(task: TaskClass) {
		if (task.id) {
			const taksIndex = tasks.findIndex((t) => t.id === task.id);
			if (taksIndex === -1) {
				console.error('Something went wrong, task index is -1. Unexpected.');
			} else {
				tasks[taksIndex] = task;
			}
		} else {
			tasks.push(task);
		}
		history.back();
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
			<button
				class="single-task"
				onclick={() => {
					selectedTask = i;
					pushState('', {
						projectEditMode: true,
						showTaskPopup: true
					});
				}}
			>
				{task.title}
			</button>
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
		width: 100%;
		text-align: left;
		background-color: transparent;
		border: 2px solid var(--border);

		&:hover {
			background-color: var(--hover-color);
		}
	}
</style>
