<script lang="ts">
	import { goto } from '$app/navigation';
	import Task from '$lib/components/Task.svelte';
	import { tasksStore } from '$lib/states.svelte';
	let { projectId }: { projectId?: string } = $props();
	const taskUrl = projectId ? `/project/${projectId}/task` : '/project/task';

	async function openTask(index: number) {
		tasksStore.selected = index;
		await goto(taskUrl);
	}

	async function addTask() {
		await goto(taskUrl);
	}
	let modal: HTMLDialogElement;
	let content = $state('');
</script>

<div class="headline">
	<h2>Tasks</h2>
</div>
<div class="tasks-container">
	{#each tasksStore.tasks as task, i}
		<Task taskObject={task} onclick={() => openTask(i)}></Task>
	{/each}
	<button onclick={addTask} class="add-task">Add Task</button>
</div>

<style>
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
		margin: auto;
		padding: 1rem;
		border: 1px solid #eee;
		border-radius: 5px;
		width: 50%;
		max-width: 1000px;
	}
	.add-task {
		background-color: #fcfcfc;
		padding: 0.5rem;
		cursor: pointer;
		width: 100%;
	}
	.tasks-container {
		display: flex;
		flex-wrap: nowrap;
		flex-direction: column;
		gap: 0.5rem;
	}
	.headline {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
</style>
