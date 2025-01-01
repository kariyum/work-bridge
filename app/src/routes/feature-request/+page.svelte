<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { formatDate } from '$lib/utils.js';

	type FeatureRequestPOST = {
		title: string;
		description: string;
	};

	let formHtmlElement: HTMLFormElement;
	async function postFeatureRequest(event: Event) {
		event.preventDefault();
		const formData = new FormData(formHtmlElement);
		const payload = Object.fromEntries(formData.entries()) as FeatureRequestPOST;

		const response = await fetch('/api/feature-request', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});

		if (response.ok) {
			await invalidate('/api/feature-request');
			console.log('Saved!');
		} else {
			console.error(response);
		}
	}

	let { data } = $props();
	let featureRequests = $derived.by(() => {
		data;
		return data.featureRequests;
	});
	$inspect(featureRequests);
</script>

<h2>Welcome to the community!</h2>
<h3>Feature requests</h3>
<form onsubmit={postFeatureRequest} bind:this={formHtmlElement}>
	<label for="title"> Title:</label>
	<input type="text" name="title" id="title" />

	<label for="description"> Description: </label>
	<input type="text" name="description" id="description" />

	<label for="new">
		<input type="checkbox" name="new" id="new" />
		I have searched and this haven't been requested before.</label
	>
	<label for="type">Request Type: </label>
	<input type="text" name="request_type" id="request_type" />

	<input type="submit" value="Submit" />
</form>

<div class="table-container">
	<table>
		<thead>
			<tr>
				<th>ID</th>
				<th>Type</th>
				<th>Title</th>
				<th>Description</th>
				<th>Status</th>
				<th>Created At</th>
			</tr>
		</thead>
		<tbody>
			{#each featureRequests as data}
				<tr>
					<td>#{data.id}</td>
					<td>Feature</td>
					<td>{data.title}</td>
					<td>{data.description}</td>
					<td>todo</td>
					<td>{formatDate(data.created_at)}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	td,
	th {
		padding: 0.5rem;
		border-bottom: 1px solid #ccc;
	}

	form {
		margin-top: 1rem;
		width: fit-content;
		gap: 0.5rem;
	}

	label {
		display: block;
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

	th {
		text-align: left;
	}

	tr:last-child td {
		border: none;
	}

	.table-container {
		border: 1px solid #ccc;
		border-radius: 5px;
	}
</style>
