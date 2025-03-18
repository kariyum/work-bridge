<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { WebSocketService } from '$lib/realtime';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type { BaseNotification, ProposalNotification, User } from '$lib/types';
	import ThemeToggler from './ThemeToggler.svelte';
	import NotificationMenu from './NotificationMenu.svelte';
	import Notifications from './Notifications.svelte';
	import Toast from './Toast.svelte';

	let { user, notifications }: { user: User; notifications: BaseNotification[] } = $props();
	let webSocketService: WebSocketService;

	onMount(() => {
		if (browser) {
			webSocketService = WebSocketService.getInstance();
			webSocketService.subscribeToProposalNotifications((notif) => {
				const id = Date.now();
				toastsQueue.push({
					id,
					notification: notif,
					remove: () => {
						toastsQueue = toastsQueue.filter((p) => p.id !== id);
					}
				});
			});
		}
	});

	export interface ToastInterface {
		id: number;
		notification: BaseNotification;
		remove: () => void;
	}

	let toastsQueue: ToastInterface[] = $state([]);
	function showToast() {
		const id = Date.now();
		toastsQueue.push({
			id,
			notification: {
				notification_type: 'proposal',
				content: {
					proposal_id: 'Proposal #1',
					proposal_status: 'approved'
				},
				created_at: new Date(Date.now()),
				id: 2013,
				user_id: 'this user'
			} as ProposalNotification,
			remove: () => {
				toastsQueue = toastsQueue.filter((p) => p.id !== id);
			}
		});
	}

	async function logout() {
		const response = await fetch('/api/auth/logout');

		if (response.ok) {
			WebSocketService.getInstance().close();
			await goto('/', { invalidateAll: true });
		}
	}

	let showNotifications = $state(false);
	let dropdownModal: HTMLDivElement;

	function notificationClickHandler(event: MouseEvent) {
		if ((event.target as Element)?.closest('.notification-container')) {
			return;
		}
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
				<button onclick={() => showToast()}>Show Toast</button>
				<button onclick={() => invalidateAll()}>Refresh</button>

				{#if user.role === 'recruiter'}
					<li><a href="/project">Create a project</a></li>
				{/if}
				<li><a href="/messages">Discussions</a></li>
				<li class="notifications">
					<button> Notifications </button>
					<div class="notification-container" class:showNotifications bind:this={dropdownModal}>
						<NotificationMenu {notifications} />
					</div>
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
	:global(.theme-icons) {
		width: 1.2rem;
	}

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

	.showNotifications {
		display: block !important;
	}

	.notifications {
		position: relative;
	}

	.notification-container {
		display: none;
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
