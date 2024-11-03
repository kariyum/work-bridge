<script lang="ts">
	import { userStore } from '$lib/storage.js';
	import { onDestroy } from 'svelte';
	import { tick } from 'svelte';

	const { data } = $props();

	let groupId = $state('a9f734b8-090d-4765-8fb0-13e6accf15bd');
	let url = $derived(`/api/chat/${groupId}`);
	let webSocket: WebSocket;
	let message: string = $state('');
	let smoothScroll: boolean = false;
	let viewport: HTMLDivElement;

	let localMessages: Array<MessagesJsonResponse> = $state([]);
	let messages: Array<MessagesJsonResponse> = $derived(data.messages.concat(localMessages));

	// $effect is used here to clear localMessages when data.messages changes;
	// meaning when the user clicks on another discussion
	$effect.pre(() => {
		data.messages;
		localMessages = [];
		smoothScroll = false;
		if (viewport) {
			// reset the scroll position
			viewport.scrollTo({
				left: 0,
				top: 0,
				behavior: 'instant'
			});
		}
	});

	function toClientMessage(str: string) {
		let res: ClientMessage = {
			disucssion_id: parseInt(data.discussion_id),
			content: str
		};
		return JSON.stringify(res);
	}

	function onClick(event: Event) {
		event.preventDefault();
		if (message.length == 0 || message.trim().length == 0) {
			return;
		}
		webSocket.send(toClientMessage(message));
		if ($userStore?.email == undefined) {
			console.log('Email is undefined');
			return;
		}
		const msg: MessagesJsonResponse = {
			id: Math.random(),
			from_user_id: $userStore?.email,
			content: message,
			created_at: new Date().toISOString()
		};
		console.log('SENDING MESSAGE', message);
		localMessages.push(msg);
		message = '';
	}

	$effect.pre(() => {
		messages;
		// const autoscroll =
		// 	viewport && viewport.offsetHeight + viewport.scrollTop > viewport.scrollHeight - 100;
		if (viewport) {
			tick().then(() => {
				viewport.scrollTo({
					left: 0,
					top: viewport.scrollHeight,
					behavior: smoothScroll ? 'smooth' : 'instant'
				});
				smoothScroll = true;
			});
		}
	});
	import { onMount } from 'svelte';
	onMount(() => {
		viewport.scrollTo({ left: 0, top: viewport.scrollHeight, behavior: 'instant' });
		smoothScroll = true;
		webSocket = new WebSocket(url);
		webSocket.onmessage = function (event) {
			// console.log('RECEIVED MESSAGE', event.data);
			const data = JSON.parse(event.data);
			const msg: MessagesJsonResponse = {
				id: Math.random(),
				from_user_id: data.sender_id, // TODO: get the sender from the server
				content: data.content,
				created_at: new Date().toISOString()
			};
			if (msg.from_user_id != $userStore?.email) {
				localMessages.push(msg);
			}	
		};
	});
	onDestroy(() => {
		webSocket.close();
	});
</script>

<div class="messages" bind:this={viewport}>
	{#each messages as message}
		<div
			class="message"
			data-sender={message.from_user_id == $userStore?.email ? 'me' : message.from_user_id}
		>
			{#if message.from_user_id != $userStore?.email}
				<p style="background-color: inherit;">
					{message.from_user_id}:
				</p>
			{/if}
			{message.content}
		</div>
	{/each}
</div>
<div class="input">
	<form onsubmit={onClick} class="input-form">
		<input type="text" bind:value={message} />
		<input type="submit" value="Send" />
	</form>
</div>

<style>
	.component {
		width: 100%;
		height: 80vh;
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
		max-width: 70ch;
		word-wrap: break-word;
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
