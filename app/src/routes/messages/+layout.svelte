<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/storage.js';

	let { data, children } = $props();
	if (data.status === 401) {
		authStore.set(false);
		goto('/');
	}
</script>

<div class="component">
	<div class="container">
		<div class="discussions">
			<h1 class="header">Discussions</h1>
			{#each data.discussions as discussion}
				<p><a href="/messages/{discussion.id}">{discussion.title}</a></p>
			{/each}
		</div>
		<div class="messages-col">
            {@render children()}
		</div>
	</div>
</div>

<style>
	.component {
		width: 100%;
		height: 90vh;
		display: flex;
	}

	.container {
		display: flex;
		width: 90%;
        margin: auto;
		height: 100%;
		gap: 1rem;
	}

	.discussions {
		flex: 1;
		padding: 1rem;
		overflow-y: auto;
        border: 1px solid rgb(204, 204, 204);
        border-radius: 5px;
	}

	.messages-col {
		flex: 3;
		display: flex;
		flex-direction: column;
		background-color: #fff;
	}

	.header {
		font-size: 1.5rem;
		margin-bottom: 1rem;
		color: #333;
        width: min-content;
        margin: auto;
	}

	.messages {
		flex-grow: 1;
		overflow-y: auto;
		padding: 1rem;
		border: 1px solid #ddd;
		border-radius: 5px;
		background-color: #f7f7f7;
	}

	.message {
		padding: 0.75rem;
		border-radius: 9px;
		background-color: #e0e0e0;
		width: fit-content;
		max-width: 70%;
	}

	.message[data-sender='me'] {
		margin-left: auto;
		background-color: #007bff;
		color: #fff;
	}

	.message {
		margin: 0.5rem 0.5rem 0.5rem 0.5rem;
	}

	.input {
		display: flex;
		width: 100%;
		padding-top: 1rem;
	}

	.input-form {
		display: flex;
		width: 100%;
	}

	.input-form input[type='text'] {
		flex: 1;
		padding: 0.75rem;
		border: 1px solid #ccc;
		border-radius: 5px;
		margin-right: 0.5rem;
	}

	.input-form input[type='submit'] {
		padding: 0.75rem 1rem;
		border-radius: 5px;
		cursor: pointer;
	}

</style>
