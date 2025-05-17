<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import { TaskClass } from '$lib/components/task/states.svelte';
	import type { ProjectForm, ProjectGET, ProjectPOST } from '$lib/types/project';
	import type { TaskPOST } from '$lib/types/task';
	import RichTextEditor from '../texteditor/RichTextEditor.svelte';
	import Tasks from '../task/Tasks.svelte';
	import AsyncButton from '../button/AsyncButton.svelte';

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
		await new Promise((resolve, reject) => {
			setTimeout(() => resolve(''), 3000);
		});
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
				await goto(`/project/${projectIn.id}`);
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

<div class="container">
	{#if projectIn}
		<h1>Update your project</h1>
	{:else}
		<h1>Create a project</h1>
	{/if}

	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<form
		onsubmit={(event) => {
			event.preventDefault();
			event.stopImmediatePropagation();
		}}
		onkeypress={(event) => {
			if (event.key === 'Enter') {
				event.preventDefault();
			}
		}}
	>
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
			<RichTextEditor bind:x={projectFormInput.content}></RichTextEditor>
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

		<div style="width: 100%;">
			<Tasks projectId={projectIn?.id} bind:tasks></Tasks>
		</div>
		<hr />
		<div class="action-buttons">
			<button class="cancel-btn" onclick={() => history.back()}>Cancel</button>
			{#snippet submitButton()}
				<div>{projectIn ? 'Update Project' : 'Save Project'}</div>
			{/snippet}
			<!-- <button type="submit" onclick={(event) => handleSubmit(event)}
				>{projectIn ? 'Update Project' : 'Save Project'}</button
			> -->
			{#snippet endView()}
				<button>Done!</button>
			{/snippet}
			<AsyncButton idleView={submitButton} {endView} onclick={(event) => handleSubmit(event)} />
		</div>
		<!-- <input style="background-color:#f0f0f0;" type="submit" value="Create project" /> -->
	</form>
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

	.container {
		max-width: 1200px;
		margin: 1rem auto;
		width: 100%;
	}
	form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 0.3rem;
	}

	input {
		padding: 0;
		padding: 0.7rem;
		border: 2px solid var(--border);
	}
</style>
