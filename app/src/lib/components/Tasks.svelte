<script lang="ts">
	import { goto, pushState } from '$app/navigation';
	import { Plus } from 'lucide-svelte';

	import Task from '$lib/components/Task.svelte';
	import TaskForm from './TaskForm.svelte';
	import type { TaskGET } from '$lib/types/task';
	import { page } from '$app/state';
	import { TaskClass } from '$lib/states.svelte';

	let {
		projectId,
		tasks = $bindable()
	}: {
		projectId?: number;
		tasks: TaskClass[];
	} = $props();

	function addNewTask() {
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
				console.log(taksIndex);
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
	<button onclick={addNewTask} class="add-task"><Plus /></button>
</div>
<div class="tasks-container">
	{#if tasks.length === 0}
		<p>Add tasks by clicking on the plus (+) button.</p>
	{:else}
		{#each tasks as task, i}
			<Task
				taskObject={task}
				onclick={() => {
					selectedTask = i;
					pushState('', {
						projectEditMode: true,
						showTaskPopup: true
					});
				}}
			></Task>
		{/each}
	{/if}
</div>

{#if page.state.showTaskPopup}
	<TaskForm taskInput={tasks[selectedTask]} onSubmit={(task) => onSubmit(task)} />
{/if}

<style>
	.add-task {
		padding: 0.2rem;
		cursor: pointer;
		line-height: 0;
	}
	.tasks-container {
		display: flex;
		flex-wrap: nowrap;
		flex-direction: column;
		gap: 0.5rem;
	}
	.headline {
		padding: 0.5rem 0rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
</style>
