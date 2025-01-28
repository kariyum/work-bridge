<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import { formatDate } from '$lib/utils.js';
	import { ChevronLeft, ChevronRight, ExternalLink, Plus } from 'lucide-svelte';

	type FeatureRequestPOST = {
		title: string;
		description: string;
	};
	let dialogElement: HTMLDialogElement;
	let formHtmlElement: HTMLFormElement;
	async function postFeatureRequest(event: Event) {
		event.preventDefault();
		const formData = new FormData(formHtmlElement);
		formHtmlElement.reset();
		const payload = Object.fromEntries(formData.entries()) as FeatureRequestPOST;

		const response = await fetch('/api/feature-request', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});

		if (response.ok) {
			dialogElement.close();
			await invalidate('/api/feature-request');
		} else {
			console.error(response);
		}
	}

	function addNewFeature() {
		dialogElement.showModal();
	}

	let { data } = $props();
	const pageSize = 15;
	let startIndex = $state(0);
	let endIndex = $derived(startIndex + pageSize);

	let featureRequests = $derived.by(() => {
		return data.featureRequests.slice(startIndex, endIndex);
	});

	function handlePrevious() {
		startIndex = Math.max(0, startIndex - pageSize);
	}

	const canGoNext = () => startIndex < data.featureRequests.length - pageSize;

	function handleNext() {
		if (canGoNext()) {
			startIndex = Math.min(data.featureRequests.length, startIndex + pageSize);
		}
	}
</script>

<div class="container">
	<div class="request-feature-action">
		<h2>Feature Requests</h2>
		<dialog bind:this={dialogElement}>
			<form class="post-form" bind:this={formHtmlElement}>
				<h2>New Feature Request</h2>
				<label for="title"> Title:</label>
				<input type="text" name="title" id="title" />

				<label for="description"> Description: </label>
				<textarea maxlength="255" name="description" id="description"></textarea>
				<label for="new">
					<input type="checkbox" name="new" id="new" />
					I have searched and this haven't been requested before.</label
				>

				<div class="actions">
					<button type="submit" formmethod="dialog">Cancel</button>
					<input type="submit" onclick={postFeatureRequest} class="submit-btn" value="Submit" />
				</div>
			</form>
		</dialog>
	</div>

	<div class="table-actions">
		<div class="tfoot">
			<div>
				<span>{startIndex + 1} - {endIndex + 1} - Total {data.featureRequests.length}</span>
			</div>
			<button
				class="no-line-height paging-btn"
				title="Previous"
				onclick={handlePrevious}
				disabled={startIndex <= 0}><ChevronLeft size="24" /></button
			>
			<button
				class="no-line-height paging-btn"
				title="Next"
				onclick={handleNext}
				disabled={!canGoNext()}><ChevronRight size="24" /></button
			>
		</div>
		<button
			type="button"
			class="no-line-height add-btn"
			onclick={addNewFeature}
			title="Request new feature"><Plus size="24" /></button
		>
	</div>
	<table>
		<thead>
			<tr>
				<th>ID</th>
				<th>Title</th>
				<th>Description</th>
				<th>Status</th>
				<th>Up votes</th>
				<th>Down votes</th>
				<th>Created At</th>
				<th></th>
			</tr>
		</thead>
		<tbody>
			{#each featureRequests as data}
				{@const expansionUrl = `/feature-request/${data.id}`}
				<tr>
					<td>#{data.id}</td>
					<td style="width: 20%;">{data.title}</td>
					<td style="width: 30%;">{data.description}</td>
					<td>
						<div class="bloc" data-type="todo">todo</div>
					</td>
					<td>0</td>
					<td>0</td>
					<td>{formatDate(data.created_at)}</td>
					<td>
						<a href={expansionUrl}>
							<ExternalLink size="18" />
						</a>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	a {
		color: canvasText;
	}
	.bloc {
		width: fit-content;
		padding: 0.3rem 0.7rem;
		line-height: 1;
		border-radius: 30px;
	}

	.bloc[data-type='todo'] {
		background-color: var(--green-bg);
	}

	.bloc[data-type='in_progress'] {
		background-color: var(--violet-bg);
	}

	.bloc[data-type='done'] {
		background-color: var(--grey);
	}

	.bloc[data-type='declined'] {
		background-color: var(--red);
	}

	.submit-btn {
		background-color: var(--blue);
	}

	.table-actions {
		display: flex;
		justify-content: space-between;
		margin-top: 0.5rem;
	}

	.paging-btn {
		padding: 0.2rem;
	}

	.add-btn {
		margin: 0;
		padding: 0.2rem;
		width: 3rem;
		height: 2rem;
		background-color: var(--blue);
	}

	.container {
		max-width: 1100px;
		margin: 1rem auto auto auto;
	}

	h2 {
		font-weight: 500;
		font-size: larger;
	}
	.post-form .actions {
		display: flex;
		gap: 0.5rem;
	}

	dialog {
		margin: auto;
		padding: 1.3rem;
		border: 1px solid var(--border);
		border-radius: 7px;
		color: var(--text-color);
	}

	.request-feature-action {
		display: flex;
	}

	.request-feature-action button {
		margin-left: auto;
		width: fit-content;
	}

	td,
	th {
		padding: 0.5rem;
		border-bottom: 1px solid var(--border);
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: fit-content;
		gap: 0.5rem;
		min-width: 50ch;
	}

	form label {
		display: block;
		font-weight: 400;
		font-size: large;
	}

	form input[type='submit'] {
		cursor: pointer;
	}

	form textarea {
		resize: none;
	}

	textarea:invalid {
		border: 2px solid red;
	}

	input[type='submit'] {
		display: block;
	}

	table {
		width: 100%;
		margin: auto;
		padding: 0 0.5rem 0rem 0.5rem;
		border-collapse: collapse;
	}

	.tfoot {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: fit-content;
	}

	.tfoot button {
		aspect-ratio: 1 / 1;
		line-height: 0;
	}

	th {
		text-align: left;
	}

	tr:last-child td {
		border: none;
	}
</style>
