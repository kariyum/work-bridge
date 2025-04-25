<script lang="ts">
	import { goto, invalidate, invalidateAll } from '$app/navigation';
	import { WebSocketService } from '$lib/websocketservice';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type { BaseNotification, NewProposalNotification, ProposalNotification, User } from '$lib/types';
	import ThemeToggler from '../utility/ThemeToggler.svelte';
	import NotificationMenu from '../notification/NotificationMenu.svelte';
	import Toast from '../notification/Toast.svelte';

	let { user, notifications }: { user: User; notifications: BaseNotification[] } = $props();
	let webSocketService: WebSocketService;
	let realtimeNotifications: BaseNotification[] = $state([]);
	let finalNotifications = $derived.by(() => {
		return realtimeNotifications.concat(notifications);
	});

	onMount(() => {
		if (browser) {
			webSocketService = WebSocketService.getInstance();
			const handler = (notif: ProposalNotification | NewProposalNotification) => {
				realtimeNotifications = [notif, ...realtimeNotifications];
				const id = Date.now();
				toastsQueue.push({
					id,
					notification: notif,
					remove: () => {
						toastsQueue = toastsQueue.filter((p) => p.id !== id);
					}
				});
			};
			webSocketService.subscribeToProposalNotifications(async (notif) => {
				handler(notif);
				await invalidate(`/api/projects/${notif.content.project_id}`);
			});
			webSocketService.subscribeToNewProposalNotifications(async (notif) => {
				handler(notif);
				await invalidate(`/api/projects/${notif.content.project_id}/${notif.content.task_id}`);
			});
		}
	});

	export interface ToastInterface {
		id: number;
		notification: BaseNotification;
		remove: () => void;
	}

	let toastsQueue: ToastInterface[] = $state([]);
	async function logout() {
		const response = await fetch('/api/auth/logout');

		if (response.ok) {
			WebSocketService.getInstance().close();
			await goto('/', { invalidateAll: true });
		}
	}

	let showNotifications = $state(false);

	function notificationClickHandler(event: MouseEvent) {
		// if ((event.target as Element)?.closest('.notification-container')) {
		// 	return;
		// }
		if ((event.target as Element)?.closest('.notifications')) {
			showNotifications = !showNotifications;
		} else {
			showNotifications = false;
		}
	}
</script>

<svelte:body onclick={notificationClickHandler} />
<section>
	<div class="container">
		<h1 style="display:inline-flex; gap:1rem; justify-content:stretch;">
			<!-- <a href="/">Word-bridge</a> -->
			<!-- <div style="width: 2px; border: 1px solid black;display:inline;background-color:black;"></div> -->
			<a class="home" href="/">
				{user.role.toUpperCase()}
			</a>
		</h1>
		<nav>
			<ul>
				{#if user.role === 'recruiter'}
					<li><a href="/project">Create a project</a></li>
				{/if}
				<li><a href="/messages">Discussions</a></li>
				<li class="notifications">
					<button> Notifications </button>
					{#if showNotifications}
						<div class="notification-container">
							<NotificationMenu notifications={finalNotifications} />
						</div>
					{/if}
				</li>
				<li><a href="/settings">Settings</a></li>
				<li><a href="/feature-request">Feature Requests</a></li>
				<li>
					<button onclick={logout}> Logout </button>
				</li>
				<li class="lh-0">
					<ThemeToggler />
				</li>
			</ul>
		</nav>
	</div>
</section>

<Toast bind:toasts={toastsQueue} />

<style>
	.lh-0 {
		line-height: 0;
	}

	.home {
		position: relative;
		color: inherit;
	}

	section {
		padding: 0.4rem 0rem;
		background-color: var(--navbar-color);
		border-bottom: 1px solid var(--navbar-border);
	}

	li,
	li > * {
		color: inherit;
	}

	button {
		background-color: transparent;
		border: none;
		margin: 0;
		padding: 0;
		font-weight: normal;
		font-size: medium;

		&:hover {
			background-color: transparent;
		}
	}

	.notifications {
		position: relative;
	}

	.notification-container {
		display: block;
		position: absolute;
		border: 2px solid var(--border);
		border-radius: 5px;
		padding: 1rem;
		z-index: 1;
		width: 25rem;
		max-height: 30rem;
		overflow-y: auto;
		top: 2rem;
		transform: translateX(-20%);
		background-color: var(--background-color);
	}

	nav {
		margin-left: auto;
	}

	a {
		text-decoration: none;
	}

	nav > ul {
		display: flex;
		justify-content: flex-end;
		align-items: safe center;
		gap: 1rem;
		list-style: none;
	}

	.container {
		display: flex;
		flex-direction: row nowrap;
		align-items: safe center;
		max-width: var(--max-width);
		height: max-content;
		margin: auto;
		padding: 0.5rem var(--page-padding);
		border-radius: 7px;
	}

	ul {
		display: flex;
		justify-content: space-around;
		align-items: safe center;
		list-style: none;
		padding: 0;
		margin: auto;
	}

	li,
	button {
		height: fit-content;
		font-size: large;
	}
</style>
