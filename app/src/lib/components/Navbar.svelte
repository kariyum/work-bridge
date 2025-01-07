<script lang="ts">
	import { goto } from '$app/navigation';
	import { Bell, Moon, Sun } from 'lucide-svelte';
	import { LogOut } from 'lucide-svelte';
	import { WebSocketService } from '$lib/realtime';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

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
	function getTheme() {
		if (browser) {
			return (
				localStorage.getItem('app:theme') ??
				(window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
			);
		}
		return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
	}

	let theme: string | undefined = $state();

	const toggleTheme = () => {
		if (theme) {
			document.documentElement.classList.remove(theme);
			theme = theme === 'light' ? 'dark' : 'light';
			document.documentElement.classList.add(theme);
			localStorage.setItem('app:theme', theme);
		}
	};

	onMount(() => {
		theme = getTheme();
	});
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
				<!-- <li><a href="/"><span class="material-symbols-outlined">home</span></a></li> -->
				<li>
					<a href="/profile"> {user.email}</a>
				</li>
				<li><a href="/project">Create a project</a></li>
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
					<button onclick={toggleTheme} class="lh-0 outline-none" aria-label="Toggle dark mode" aria-pressed={theme === 'dark'}> </button>
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
		background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/></svg>');
		aspect-ratio: 1 / 1;
		background-size: cover;
		width: 1.2rem;
		line-height: 0;

		:global(.dark) & {
			background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/></svg>');
		}
	}

	.outline-none {
		outline: none;
	}
	.home {
		/* padding: 0 0.5rem; */
		position: relative;
		/* background-color: var(--orange);
		box-shadow: 0 0 1rem var(--orange); */
		/* border-color: var(--orange); */
		/* border-radius: 3px; */
	}

	section {
		padding: 0.3rem 0;
	}

	button {
		background-color: transparent;
		border: none;
		cursor: pointer;
		margin: 0;
		padding: 0;
	}

	.showNotifications {
		display: block !important;
	}

	.notification-container {
		display: none;
		position: absolute;
		background-color: var(--bg);
		color: var(--dark-text);
		border: 1px solid var(--border);
		border-radius: 5px;
		padding: 1rem;
		z-index: 1;
		width: 15rem;
		transform: translateX(-35%);
	}

	nav {
		margin-left: auto;
	}

	a {
		text-decoration: none;
		color: var(--dark-text);
	}

	nav > ul {
		display: flex;
		justify-content: flex-end;
		align-items: safe center;
		gap: 1rem;
		list-style: none;
	}
	.container {
		/* border: 2px solid black; */
		display: flex;
		flex-direction: row nowrap;
		align-items: safe center;
		/* max-width: 1500px; */
		height: max-content;
		margin: auto;
		padding: 0.5rem 0rem;
		/* border: 2px solid #eee; */
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
		font-size: medium;
	}
</style>
