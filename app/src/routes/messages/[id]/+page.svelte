<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import { tick } from 'svelte';
	import { SendHorizontal } from 'lucide-svelte';
	import { WebSocketService } from '$lib/realtime';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type { ClientMessage, MessagesJsonResponse } from '$lib/types.js';

	const { data } = $props();

	let webSocketService: WebSocketService;
	let unsubscribe: () => void;
	let message: string = $state('');
	let smoothScroll: boolean = false;
	let justSwitched: boolean = true;
	let viewport: HTMLDivElement;
	let receivers: string[] = $derived.by(() => {
		data.discussion_id;
		data.discussions;
		const maybeDiscussion = data.discussions?.find(
			(discussion) => discussion.id.toString() == data.discussion_id
		);
		return maybeDiscussion?.user_ids.filter((user_id) => user_id != data.user?.email) ?? [];
	});

	let localMessages: Array<MessagesJsonResponse> = $derived.by(() => {
		data.messages;
		let result = $state([]);
		return result;
	});
	let messages: Array<MessagesJsonResponse> = $derived(data.messages.concat(localMessages));
	
	onMount(() => {
		if (browser) {
			webSocketService = WebSocketService.getInstance();
			unsubscribe = webSocketService.subscribe((msg) => {
				console.log(`msg from user id: ${msg.from_user_id}, data.user.email ${data.user?.email}, from_user_id != data.user?.email ${msg.from_user_id != data.user?.email}`);
				if (parseInt(data.discussion_id) == msg.discussion_id) {
					localMessages.push(msg);
				}
			});
		}
	});

	onDestroy(() => {
		if (browser) {
			unsubscribe();
		}
	});

	function toClientMessage(content: string) {
		let res: ClientMessage = {
			discussion_id: parseInt(data.discussion_id),
			content: content,
			receivers: receivers
		};
		return JSON.stringify(res);
	}

	function onClick(event: Event) {
		event.preventDefault();
		if (message.length == 0 || message.trim().length == 0) {
			return;
		}
		try {
			webSocketService.send(toClientMessage(message));
		} catch (err) {
			console.error("Failed to push message upstream...");
		}
		if (data.user?.email == undefined) {
			console.log('Email is undefined');
			return;
		}
		const msg: MessagesJsonResponse = {
			id: Math.random(),
			from_user_id: data.user?.email,
			content: message,
			discussion_id: parseInt(data.discussion_id),
			created_at: new Date().toISOString()
		};
		localMessages.push(msg);
		message = '';
	}

	// $effect is used here to clear localMessages when data.messages changes;
	// meaning when the user clicks on another discussion
	$effect.pre(() => {
		data.messages;
		smoothScroll = false;
		justSwitched = true;
	});

	$effect.pre(() => {
		messages;
		const autoscroll = viewport && Math.abs(viewport.scrollTop) < 100;
		if (viewport && (autoscroll ||  justSwitched)) {
			if (viewport.scrollTop == 0) {
				viewport.scrollTo({
					left: 0,
					top: -1,
					behavior: 'instant'
				});
			}
			tick().then(() => {
				viewport.scrollTo({
					left: 0,
					top: 0,
					behavior: smoothScroll ? 'smooth' : 'instant'
				});
				smoothScroll = true;
				justSwitched = false;
			});
		}
	});
</script>

<div class="outer-container" bind:this={viewport}>
	<div class="messages">
		{#each messages as message}
			<div
				class="message"
				data-sender={message.from_user_id == data.user?.email ? 'me' : message.from_user_id}
			>
				{message.content}
			</div>
		{/each}
	</div>
</div>

<div class="input">
	<form onsubmit={onClick} class="input-form">
		<input type="text" bind:value={message} placeholder="Type a message... as {data.user?.email}" />
		<button type="submit" class="icon">
			<SendHorizontal />
		</button>
	</form>
</div>

<style>
	.icon {
		background-color: transparent;
		border: none;
		line-height: 0;
	}

	.outer-container {
		display: flex;
		overflow-y: auto;
		flex-direction: column-reverse;
		scroll-behavior: smooth;
		flex: 1;
	}

	.messages {
		padding: 1rem;
		width: 100%;
		flex-grow: 1;
		flex: 1;
		background-color: canvas;
	}

	.message {
		padding: 0.75rem;
		border-radius: 9px;
		background-color: var(--message-bg);
		width: fit-content;
		max-width: 70ch;
		word-wrap: break-word;
	}

	.message[data-sender='me'] {
		margin-left: auto;
	}

	.message {
		margin: 0.5rem 0.5rem 0.5rem 0.5rem;
	}

	.input {
		display: flex;
		width: 100%;
	}

	.input-form {
		display: flex;
		width: 100%;
		border-top: 2px solid var(--border);
	}

	.input-form input[type='text'] {
		flex: 1;
		padding: 0.75rem;
		border-radius: 0;
		border: 0;
		outline: none;
	}

	.input-form button[type='submit'] {
		padding: 0.75rem 1rem;
		cursor: pointer;
		border-left: 0;
		border-top: 0;
		border-radius: 0;
		background-color: var(--input-bg);
	}
</style>
