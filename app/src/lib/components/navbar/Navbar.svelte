<script lang="ts">
	import { goto, invalidate, invalidateAll } from '$app/navigation';
	import { WebSocketService } from '$lib/websocketservice';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type {
		BaseNotification,
		NewProposalNotification,
		ProposalNotification,
		User
	} from '$lib/types';
	import ThemeToggler from '../utility/ThemeToggler.svelte';
	import NotificationMenu from '../notification/NotificationMenu.svelte';
	import Toast from '../notification/Toast.svelte';
	import {
		Bell,
		CirclePlus,
		Home,
		LogOut,
		Menu,
		MessageCircle,
		PackagePlus,
		Settings
	} from 'lucide-svelte';

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
	let menuDialog: HTMLDialogElement;

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

	function createMobileState() {
		let showMenu = $state(false);
		return {
			openMenu: () => {
				menuDialog.showModal();
				showMenu = true;
				if (browser && document) {
					document.body.style.overflow = 'hidden';
				}
			},
			closeMenu: () => {
				menuDialog.close();
				showMenu = false;
				if (browser && document) {
					document.body.style.overflow = 'auto';
				}
			},
			showMenu: () => showMenu
		};
	}

	let mobileState = createMobileState();
</script>

<svelte:body
	onclick={(event) => {
		notificationClickHandler(event);
	}}
/>

{#snippet menuMobile()}
	<ul>
		<div class="mobile">
			<div class="title">
				<li>
					<div>Work Bridge</div>
					<ThemeToggler />
				</li>
				<hr />
			</div>
		</div>
		<div class="mobile">
			<li>
				<a href="/"
					><Home />
					<div>Home</div></a
				>
			</li>
		</div>
		{#if user.role === 'recruiter'}
			<li>
				<a href="/project"
					><CirclePlus />
					<div>Create a project</div></a
				>
			</li>
		{/if}
		<li>
			<a href="/messages"
				><MessageCircle />
				<div>Discussions</div></a
			>
		</li>
		<div class="mobile">
			<li>
				<a href="/notifications"
					><Bell />
					<div>Notifications</div></a
				>
			</li>
		</div>
		<li>
			<a href="/settings"
				><Settings />
				<div>Settings</div></a
			>
		</li>
		<li>
			<a href="/feature-request"
				><PackagePlus />
				<div>Feature Requests</div></a
			>
		</li>
		<li>
			<button onclick={logout}
				><LogOut />
				<div>Logout</div>
			</button>
		</li>
	</ul>
{/snippet}

{#snippet menuDesktop()}
	<ul>
		<div class="mobile">
			<div class="title">
				<li>Work Bridge</li>
				<hr />
			</div>
		</div>
		<div class="mobile">
			<li><a href="/"><Home /> Home</a></li>
		</div>
		{#if user.role === 'recruiter'}
			<li><a href="/project">Create a project</a></li>
		{/if}
		<li><a href="/messages">Discussions</a></li>
		<div class="desktop">
			<li class="notifications">
				<button> Notifications </button>
				{#if showNotifications}
					<div class="notification-container">
						<NotificationMenu notifications={finalNotifications} />
					</div>
				{/if}
			</li>
		</div>
		<div class="mobile">
			<li><a href="/notifications">Notifications</a></li>
		</div>
		<li><a href="/settings">Settings</a></li>
		<li><a href="/feature-request">Feature Requests</a></li>
		<li>
			<button onclick={logout}> Logout </button>
		</li>

		<li class="lh-0">
			<ThemeToggler />
		</li>
	</ul>
{/snippet}

<section>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div class="container">
		<h1 class="header">
			<div class="mobile">
				<button class="menu-icon" onclick={() => mobileState.openMenu()}>
					<Menu size="28" />
				</button>
			</div>
			<a class="home" href="/">
				{user.role.toUpperCase()}
			</a>
		</h1>
		<div class="desktop">
			<nav>
				{@render menuDesktop()}
			</nav>
		</div>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div class="mobile">
			<dialog bind:this={menuDialog} onclick={() => mobileState.closeMenu()}>
				{@render menuMobile()}
			</dialog>
		</div>
	</div>
</section>

<Toast bind:toasts={toastsQueue} />

<style>
	@media (width < 600px) {
		section {
			padding: 1rem;
		}

		@keyframes fadeIn {
			0% {
				opacity: 0;
			}

			100% {
				opacity: 0.4;
			}
		}

		@keyframes fadeOut {
			0% {
				opacity: 0.4;
			}

			100% {
				opacity: 0;
			}
		}

		.title {
			color: var(--font-color);
			li {
				padding: 1rem;
				display: flex;
				align-items: center;
				justify-content: space-between;
			}

			hr {
				border: none;
				border-top: solid;
				border-color: var(--orange);
				border-width: 2px;
			}
		}

		dialog {
			position: absolute;
			top: 0;
			left: 0;
			transform: translateX(-100%);
			border-radius: 0 20px 20px 0;
			background-color: transparent;
			height: 100vh;
			z-index: 1;
			min-width: 17rem;
			border: none;
			transition-duration: 0.3s;
			transition-timing-function: ease-out;
			transition-property: display overlay;
			transition-behavior: allow-discrete;
			max-height: unset;
		}

		dialog::backdrop {
			background-color: black;
			animation: 0.3s fadeIn forwards;
		}

		dialog:not([open])::backdrop {
			animation: 0.3s fadeOut forwards;
		}

		dialog:modal {
			max-height: unset;
		}

		dialog[open] {
			display: block;
			transform: translateX(0);

			@starting-style {
				transform: translateX(-100%);
			}
		}

		li {
			text-decoration: none;
			font-weight: 600;
			font-size: larger;

			a {
				display: flex;
				align-items: center;
				gap: 1rem;
				width: 100%;
				height: 100%;
				padding: 0.3rem 0;
			}

			button {
				display: flex;
				align-items: center;
				gap: 1rem;
				width: 100%;
				height: 100%;
				padding: 0.3rem 0;
			}

			button:hover {
				background-color: transparent;
			}
		}

		button {
			background-color: transparent;
			padding: 0;
			border: none;
			font-weight: inherit;
			font-size: inherit;
			padding: 0.3rem 0;
		}

		a {
			display: block;
			text-decoration: none;
			color: var(--font-color);
		}

		ul {
			border-radius: 0 20px 20px 0;
			list-style: none;
			display: flex;
			flex-direction: column;
			gap: 0.5rem;
			overflow: auto;
			height: 100%;
			background-color: var(--navbar-color);

			li {
				padding: 0 1rem;
			}
		}

		.header {
			display: flex;
			gap: 0.5rem;
			align-items: center;
		}

		.menu-icon {
			line-height: 0;
			padding: 0;
			background-color: transparent;
			border: none;
		}
		.desktop {
			display: none;
		}
	}

	@media (width >= 600px) {
		.mobile {
			display: none;
		}

		.desktop {
			display: block;
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
			justify-content: space-between;
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
	}
</style>
