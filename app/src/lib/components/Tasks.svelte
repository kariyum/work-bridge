<script lang="ts">
	import { goto } from '$app/navigation';
	import Task from '$lib/components/Task.svelte';
	import RichTextEditor from './RichTextEditor.svelte';
	let { projectId }: { projectId?: string } = $props();
	const taskUrl = projectId ? `/project/${projectId}/task`: '/project/task';
	let tasks: Array<TaskObject> = $state([]);
	tasks.push({
		id: 1,
		title: 'Task 1',
		description: 'Description 1',
		assignee_id: 'User 1',
		status: 'Todo'
	});

	async function addTask() {
		console.log("Task url", taskUrl);
		await goto(taskUrl);
	}
	let modal: HTMLDialogElement;
	let content = $state('');
</script>

<div class="headline">
	<h2>Tasks</h2>
</div>
<div class="tasks-container">
	{#each tasks as task}
		<Task taskObject={task}></Task>
	{/each}
	<button onclick={addTask} class="add-task">Add Task</button>
	<dialog class="popover" bind:this={modal}>
		<div class="create-task">
			<h2>Add task</h2>
			<div class="input-container">
				<input type="text" placeholder="Title" />
				<RichTextEditor bind:x={content}></RichTextEditor>
				<input type="text" placeholder="Assignee" />
				<input type="text" placeholder="Skills" />
				<input type="text" placeholder="Deadline" />
				<input type="text" placeholder="Estimated Efforts" />
			</div>
			<div class="act-task">
				<button onclick={() => modal.close()}>Cancel</button>
				<button>Add</button>
			</div>
		</div>
	</dialog>
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
