<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { formatDate } from '$lib/utils.js';

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
		<button onclick={addNewFeature}>Request a new feature</button>
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
					<input type="submit" onclick={postFeatureRequest} value="Submit" />
				</div>
			</form>
		</dialog>
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
			</tr>
		</thead>
		<tbody>
			{#each featureRequests as data}
				<tr>
					<td>#{data.id}</td>
					<td style="width: 20%;">{data.title}</td>
					<td style="width: 30%;">{data.description}</td>
					<td>todo</td>
					<td>0</td>
					<td>0</td>
					<td>{formatDate(data.created_at)}</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<div class="tfoot">
		<div>
			<span>{startIndex + 1} - {endIndex + 1} - Total {data.featureRequests.length}</span>
		</div>
		<button onclick={handlePrevious} disabled={startIndex <= 0}>{'<'}</button>
		<button onclick={handleNext} disabled={!canGoNext()}>{'>'}</button>
	</div>
</div>

<style>
	h2 {
		font-weight: 500;
		font-size: larger;
	}
	.post-form .actions {
		display: flex;
		gap: 0.5rem;
	}

	.actions > * {
		background-color: #eee;
	}

	dialog {
		margin: auto;
		padding: 1rem;
		border: none;
		border-radius: 7px;
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
		border-bottom: 1px solid #ccc;
	}

	.container {
		color: var(--dark-text);
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
		margin-top: 0.5rem;
		display: flex;
		width: 100%;
		align-items: center;
		justify-content: end;
		gap: 0.5rem;
	}

	.tfoot button {
		aspect-ratio: 1 / 1;
		line-height: 0;
	}

	.tfoot button:disabled {
		background-color: #ddd;
		border: 1px solid transparent;
	}

	th {
		text-align: left;
	}

	/* tr:last-child td {
		border: none;
	} */
</style>
