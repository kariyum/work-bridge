<script lang="ts">
	import { goto } from '$app/navigation';
	import { Plus } from 'lucide-svelte';

	import Task from '$lib/components/Task.svelte';
	import type { TasksGlobalState } from '$lib/states.svelte';
	let {
		projectId,
		tasksGlobalState
	}: {
		projectId?: number;
		tasksGlobalState: TasksGlobalState;
	} = $props();
	
	const taskUrl = $derived(
		projectId ? `/project/${projectId}/task?s=${tasksGlobalState.selected}` : '/project/task'
	);

	async function openTask(index: number) {
		tasksGlobalState.selected = index;
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
	<button onclick={addTask} class="add-task"><Plus /></button>
</div>
<div class="tasks-container">
	{#if tasksGlobalState.tasks.length === 0}
		<p>Add tasks by clicking on the plus (+) button.</p>
	{:else}
		{#each tasksGlobalState.tasks as task, i}
			<Task taskObject={task} onclick={() => openTask(i)}></Task>
		{/each}
	{/if}
</div>

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
