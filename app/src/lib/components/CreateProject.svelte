<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import { TaskClass } from '$lib/states.svelte';
	import type { ProjectForm, ProjectGET, ProjectPOST } from '$lib/types/project';
	import type { TaskGET, TaskPOST } from '$lib/types/task';
	import RichTextEditor from './RichTextEditor.svelte';
	import Tasks from './Tasks.svelte';

	let {
		projectIn
	}: {
		projectIn?: ProjectGET;
	} = $props();

	let projectFormInput: ProjectForm = $derived.by(() => {
		let projectState = $state({
			title: projectIn?.title ?? '',
			content: projectIn?.content ?? '',
			budget: projectIn?.budget.toString() ?? '',
			currency_code: projectIn?.currency_code ?? '',
			deadline: projectIn?.deadline.toLocaleDateString() ?? ''
		});
		return projectState;
	});

	$effect(() => {
		tasks = projectIn?.tasks?.map((task) => TaskClass.fromGET(task)) ?? ([] as TaskClass[]);
	});

	let tasks = $state(
		projectIn?.tasks?.map((task) => TaskClass.fromGET(task)) ?? ([] as TaskClass[])
	);

	async function handleSubmit(event: Event) {
		event.preventDefault();
		const projectPost: ProjectPOST = {
			title: projectFormInput.title,
			content: projectFormInput.content,
			budget: parseFloat(projectFormInput.budget),
			currency_code: projectFormInput.currency_code,
			deadline: new Date(projectFormInput.deadline).toISOString()
		};

		const tasksPayload = tasks.map((task) => {
			const attributes: TaskPOST = {
				title: task.title,
				content: task.content,
				assignee_id: task.assignee_id,
				skills: task.skills,
				status: task.status,
				budget: parseFloat(task.budget?.toString() ?? '0'),
				deadline: new Date().toISOString()
			};
			return {
				...attributes,
				id: task.id
			};
		});

		const payload = {
			...projectPost,
			tasks: tasksPayload
		};

		if (projectIn?.id) {
			const response = await fetch(`/api/projects/${projectIn.id}`, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(payload)
			});

			if (response.status === 200) {
				await invalidate(`/api/projects/${projectIn.id}`);
				history.back();
			}
			return;
		} else {
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
	}
</script>

<div class="outer-container">
	<div class="container">
		{#if projectIn}
			<h1>Update your project</h1>
		{:else}
			<h1>Create a project</h1>
		{/if}

		<form action="" onsubmit={(event) => event.preventDefault()}>
			<div class="input">
				<label for="title">Project Title</label>
				<input
					type="text"
					id="title"
					placeholder="e.g. Business Website"
					bind:value={projectFormInput.title}
				/>
			</div>
			<div class="input">
				<label for="">Project Description</label>
				<RichTextEditor contentIn={projectIn?.content ?? ''} bind:x={projectFormInput.content}
				></RichTextEditor>
				<!-- <textarea name="content" id="content" placeholder="The project is about designing and implementing..." bind:value={content}></textarea> -->
			</div>

			<div class="input">
				<label for="">Budget</label>
				<input type="text" id="budget" placeholder="500 DT" bind:value={projectFormInput.budget} />
			</div>

			<div class="input">
				<label for="">Deadline</label>
				<input
					type="date"
					id="deadline"
					placeholder="Project deadline"
					bind:value={projectFormInput.deadline}
				/>
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

			<div style="width: 100%;">
				<Tasks projectId={projectIn?.id} bind:tasks></Tasks>
			</div>
			<hr />
			<div class="action-buttons">
				<button class="cancel-btn" onclick={() => history.back()}>Cancel</button>
				<button onclick={handleSubmit}>{projectIn ? 'Update Project' : 'Save Project'}</button>
			</div>
			<!-- <input style="background-color:#f0f0f0;" type="submit" value="Create project" /> -->
		</form>
	</div>
</div>

<style>
	.action-buttons {
		display: flex;
		gap: 1rem;
		margin-left: auto;
	}
	hr {
		width: 100%;
		margin: 1rem 0 0.3rem 0;
		border: none;
		border-top: 2px solid var(--border);
	}
	.input {
		width: 100%;
		margin: 0 0 0.5rem 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.input > input {
		width: 100%;
	}

	select {
		border-color: var(--border);
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
		gap: 0.3rem;
	}

	input,
	textarea {
		padding: 0;
		padding: 1%;
		border: 2px solid var(--border);
	}
</style>
