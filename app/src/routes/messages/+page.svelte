<script lang="ts">
	let groupId = $state('a9f734b8-090d-4765-8fb0-13e6accf15bd');
	let url = $derived(`ws://localhost:8080/${groupId}`);

	let webSocket: WebSocket;
	let message: string = $state('');
	function onClick(event: Event) {
		event.preventDefault();
		webSocket.send(message);
		message = '';
		console.log('SENDING MESSAGE', message);
	}

	let messages: Array<string> = $state(new Array());

	import { onMount } from 'svelte';

	onMount(() => {
		webSocket = new WebSocket(url);
		webSocket.onmessage = function (event) {
			console.log('RECEIVED MESSAGE', event.data);
			messages.push(event.data);
		};
	});
</script>

<div class="component">
	<div class="container">
        <div class="users-col">
            <h1 class="users-header">Discussions</h1>
        </div>
		<div class="message-col">
			<div class="messages">
				{#each messages as message}
					<p class="message">
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
    .users-header {
        margin: auto;
        width: min-content;
    }
	
    .container {
		margin: auto;
		display: flex;
		width: 100%;
		max-height: 800px;
		justify-content: space-between;
        gap: 1rem;
	}

	.users-col {
		flex-grow: 1;
        border: 2px solid rgb(211, 211, 211);
        border-radius: 5px;
	}

	.message-col {
		flex-grow: 4;
		display: flex;
		flex-direction: column;
		height: 800px;
	}
	.input {
		display: flex;
		width: 100%;
	}
	.input > form > input[type='text'] {
		width: 100%;
	}

	.messages {
		flex-grow: 1;
		max-height: 800px;
		overflow-y: auto;
        border: 2px solid rgb(211, 211, 211);
        border-radius: 5px;
        max-width: 100%;
	}

	.component {
		width: 90%;
        margin: auto;
	}

</style>
