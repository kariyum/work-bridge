<script lang="ts">
	import { goto } from '$app/navigation';
	import { tasksStore } from '$lib/states.svelte';
	import RichTextEditor from './RichTextEditor.svelte';
	import Tasks from './Tasks.svelte';

	let {
		projectId = $bindable(),
		title = $bindable(),
		content = $bindable(""),
		budget = $bindable(),
		deadline = $bindable()
	}: { projectId?: string; title?: string; content?: string; budget?: string; deadline?: Date } = $props();

	function toSimpleString(date: Date) {
		return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		const project = {
			title,
			content,
			budget: parseFloat(budget ?? "0"),
			// deadline: parseInt((Date.parse(deadline) / 1000).toFixed(0)),
			deadline: new Date().toISOString(),
			currency_code: 'TD'
		};

		const tasks = tasksStore.map(task => {
			return {
				title: task.title,
				content: task.content,
				assignee_id: task.assignee_id,
				skills: task.skills,
				status: task.status,
				budget: parseFloat(task.budget?.toString() ?? "0"),
				deadline: new Date().toISOString(),
				currency_code: 'TD'
			}
		});

		const payload = {
			...project,
			tasks
		}

		const response = await fetch('/api/projects', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});

		if (response.status === 201) {
			await goto('/');
		}
	}
</script>

<div class="outer-container">
	<div class="container">
		{#if projectId}
			<h1>Update your project</h1>
		{:else}
			<h1>Create a project</h1>
		{/if}
		<form action="" onsubmit={(event) => event.preventDefault()}>
			<div class="input">
				<label for="title">Project Title</label>
				<input type="text" id="title" placeholder="e.g. Business Website" bind:value={title} />
			</div>
			<div class="input">
				<label for="">Project Description</label>
				<RichTextEditor bind:x={content}></RichTextEditor>
				<!-- <textarea name="content" id="content" placeholder="The project is about designing and implementing..." bind:value={content}></textarea> -->
			</div>

			<div class="input">
				<label for="">Budget</label>
				<input type="text" id="budget" placeholder="500 DT" bind:value={budget} />
			</div>

			<div class="input">
				<label for="">Deadline</label>
				<input type="date" id="deadline" placeholder="Project deadline" bind:value={deadline} />
			</div>
			<!-- <div class="input">
				<label for="">Project Categories</label>
				<select name="" id="">
					<option value="" disabled selected>Select your option</option>
					<option value="kk">Web developer</option>
					<option value="kk">UI/UX designer</option>
					<option value="kk">Architect</option>
				</select>
			</div> -->

			<!-- <div class="input">
				<label for="">Start Date</label>
				<input type="text" placeholder="Start Date" />
			</div> -->
			<div style="width: 100%;">
				<Tasks projectId={projectId}></Tasks>
			</div>
			<button onclick={handleSubmit}>Submit</button>
			<!-- <input style="background-color:#f0f0f0;" type="submit" value="Create project" /> -->
		</form>
	</div>
</div>

<style>
	.input {
		width: 100%;
		margin: 0 0 0.5rem 0;
		display: flex;
		flex-direction: column;
	}

	.input > input,
	textarea,
	select {
		margin-top: 0.5rem;
		width: 100%;
	}

	select {
		border-color: rgb(197, 197, 197);
		border-radius: 5px;
		height: 2rem;
		padding: 0 0.3rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		margin: 1rem 0 1rem 0;
	}

	.outer-container {
		display: flex;
		flex-direction: column;
		margin: auto;
		width: 100%;
	}
	.container {
		max-width: 1200px;
		padding: 0% 5% 0% 5%;
		margin: auto;
		width: 100%;
	}
	form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	textarea {
		resize: none;
	}

	/* 
	*:focus {
		outline: none;
	} */

	input,
	textarea {
		padding: 0;
		padding: 1%;
		/* border: none; */
	}
</style>
