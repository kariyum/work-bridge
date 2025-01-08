<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import { tick } from 'svelte';
	import { SendHorizontal } from 'lucide-svelte';
	import { WebSocketService } from '$lib/realtime';

	const { data } = $props();

	let webSocketService: WebSocketService;
	let unsubscribe: () => void;
	let message: string = $state('');
	let smoothScroll: boolean = false;
	let viewport: HTMLDivElement;
	let receivers: string[] = $derived.by(() => {
		data.discussion_id;
		data.discussions;
		const maybeDiscussion = data.discussions.find(
			(discussion) => discussion.id.toString() == data.discussion_id
		);
		return maybeDiscussion?.user_ids.filter((user_id) => user_id != data.user?.email) ?? [];
	});

	let localMessages: Array<MessagesJsonResponse> = $state([]);
	let messages: Array<MessagesJsonResponse> = $derived(data.messages.concat(localMessages));

	onMount(() => {
		if (browser) {
			webSocketService = WebSocketService.getInstance();
		}
	})
	// $effect is used here to clear localMessages when data.messages changes;
	// meaning when the user clicks on another discussion
	$effect.pre(() => {
		data.messages;
		untrack(() => {
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
		webSocketService.send(toClientMessage(message));
		if (data.user?.email == undefined) {
			console.log('Email is undefined');
			return;
		}
		const msg: MessagesJsonResponse = {
			id: Math.random(),
			from_user_id: data.user?.email,
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
	import { browser } from '$app/environment';
	onMount(() => {
		console.log('onMount', data.user?.email);
		viewport.scrollTo({ left: 0, top: viewport.scrollHeight, behavior: 'instant' });
		smoothScroll = true;
		unsubscribe = webSocketService.subscribe((msg) => {
			console.log(msg.from_user_id, data.user?.email, msg.from_user_id != data.user?.email);
			if (msg.from_user_id != data.user?.email) {
				localMessages.push(msg);
			}
		});
	});
	onDestroy(() => {
		unsubscribe();
	});
</script>

<div class="messages" bind:this={viewport}>
	{#each messages as message}
		<div
			class="message"
			data-sender={message.from_user_id == data.user?.email ? 'me' : message.from_user_id}
		>
			{#if message.from_user_id != data.user?.email}
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
		<input type="text" bind:value={message} placeholder="Type a message..." />
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
	.messages {
		flex-grow: 1;
		overflow-y: auto;
		padding: 1rem;
		border-left: 0px;
		/* border-radius: 5px; */
	}

	.message {
		padding: 0.75rem;
		border-radius: 9px;
		background-color: var(--border);
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
	}

	.input-form input[type='text'] {
		flex: 1;
		padding: 0.75rem;
		border: 1px solid var(--border);
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
	}
</style>
