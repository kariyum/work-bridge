<script lang="ts">
	import { goto } from '$app/navigation';
	import { WebSocketService } from '$lib/realtime';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type { User } from '$lib/types';
	import ThemeToggler from './ThemeToggler.svelte';

	let { user }: { user: User } = $props();
	let webSocketService: WebSocketService;

	onMount(() => {
		if (browser) {
			webSocketService = WebSocketService.getInstance();
		}
	});

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
				{#if user.role === 'recruiter'}
					<li><a href="/project">Create a project</a></li>
				{/if}
				<li><a href="/messages">Discussions</a></li>
				<li class="notifications">
					<button> Notifications </button>
					<div class="notification-container" class:showNotifications bind:this={dropdownModal}>
						<h1>Notifications</h1>
						<div>Notification 1</div>
						<div>Notification 2</div>
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
	}

	.showNotifications {
		display: block !important;
	}

	.notification-container {
		display: none;
		position: absolute;
		border: 2px solid var(--border);
		border-radius: 5px;
		padding: 1rem;
		z-index: 1;
		width: 15rem;
		transform: translateX(-35%);
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
