<script lang="ts">
	import RichTextEditor from '$lib/components/RichTextEditor.svelte';
	import { TaskClass, tasksStore } from '$lib/states.svelte';
	import { untrack } from 'svelte';
	let { data } = $props();

	$effect(() => {
		data.selectedIndex;
		untrack(() => {
			console.log('s IS', data.selectedIndex);
			if (data.selectedIndex != undefined && data.selectedIndex != null) {
				console.log('Settings selected to ', data.selectedIndex);
				tasksStore.selected = data.selectedIndex;
				taskClass = initTaskClass();
			}
		});
	});

	function initTaskClass() {
		let result;
		if (tasksStore.selected != -1) {
			result = tasksStore.tasks[tasksStore.selected].copy();
		} else {
			result = new TaskClass('', '', '', '');
		}
		console.log('TaskClass:', result);
		return result;
	}

	function updateTaskClass() {
		if (tasksStore.selected != -1) {
			tasksStore.tasks[tasksStore.selected].assignFrom(taskClass);
		} else {
			tasksStore.tasks.push(taskClass);
		}
	}

	let taskClass = $state(initTaskClass());

	function add(event: Event) {
		event.preventDefault();
		updateTaskClass();
		tasksStore.selected = -1;
		history.back();
	}

	function cancel() {
		tasksStore.selected = -1;
		history.back();
	}
</script>

<div class="blur">
	<div class="popover">
		<div class="create-task">
			<h2>Add task</h2>
			<form class="input-container">
				<input type="text" placeholder="Title" bind:value={taskClass.title} />
				{#key taskClass}
					<RichTextEditor bind:x={taskClass.content}></RichTextEditor>
				{/key}
				<input type="text" placeholder="Assignee" bind:value={taskClass.assignee_id} />
				<input type="text" placeholder="Skills" bind:value={taskClass.skills} />
				<!-- <input type="text" placeholder="Status" bind:value={taskClass.status} /> -->
				<select name="status" id="status" bind:value={taskClass.status}>
					<option value="todo">Todo</option>
					<option value="inprogress">In Progress</option>
					<option value="done">Done</option>
				</select>
				<input type="text" placeholder="Budget" bind:value={taskClass.budget} />
				<input type="text" placeholder="Deadline" bind:value={taskClass.deadline} />
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
	.popover2 {
		margin: auto;
		padding: 1rem;
		border: 1px solid #eee;
		border-radius: 5px;
		width: 50%;
		max-width: 1000px;
	}
	.popover {
		position: absolute;
		top: 50%;
		left: 50%;
		width: 50%;
		max-width: 1000px;
		transform: translate(-50%, -50%);
		padding: 1rem;
		border: 1px solid #eee;
		border-radius: 5px;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
		/* blur */
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
</style>
