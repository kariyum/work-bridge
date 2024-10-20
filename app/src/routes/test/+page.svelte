<script lang="ts">
	let groupId = $state("a9f734b8-090d-4765-8fb0-13e6accf15bd");
	let url = $derived(`ws://localhost:8080/${groupId}`);
	let webSocket = new WebSocket(url);

	let message: string;
	function onClick(event: Event) {
        event.preventDefault();
		webSocket.send(message);
		console.log('SENDING MESSAGE', message);
	}

	let messages: Array<string> = $state(new Array());

	import { onMount } from 'svelte';

	onMount(() => {
		webSocket.onmessage = function (event) {
			console.log('RECEIVED MESSAGE', event.data);
			messages.push(event.data);
		};
	});
</script>
{url}
<h1>Connecting to '{groupId}'</h1>
<input type="text" name="group id" id="groupid" bind:value={groupId}>
<div class="component">
	<div class="container">
		<div class="users-col">USERS</div>
		<div class="message-col">
			<div class="messages">
				{#each messages as message}
					<p>
						{message}
					</p>
				{/each}
			</div>
			<div class="input">
				<form onsubmit={onClick} class="input">
					<input type="text" bind:value={message} />
					<input type="submit" value="Send" />
				</form>
			</div>
		</div>
	</div>
</div>

<style>
	.container {
		margin: auto;
		display: flex;
		width: 70%;
		max-height: 800px;
		justify-content: stretch;
	}

	.users-col {
		flex-grow: 1;
	}

	.message-col {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		justify-content: stretch;
        height: 800px;
	}
	.input {
		display: flex;
		width: 100%;
	}
	.input > form > input[type="text"] {
		width: 100%;
	}

	.messages {
		width: 100%;
		flex-grow: 1;
        max-height: 800px;
        overflow-y: auto;
	}

	.component {
		width: 100%;
	}
</style>
