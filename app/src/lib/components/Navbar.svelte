<script lang="ts">
	import { goto } from '$app/navigation';
	import { Bell } from 'lucide-svelte';
	import { LogOut } from 'lucide-svelte';

	let { user }: { user: User } = $props();
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
</script>

<svelte:body onclick={notificationClickHandler} />
<section>
	<div class="container">
		<h1 style="display:inline-flex; gap:1rem; justify-content:stretch;">
			<a href="/">Word-bridge</a>
			<div style="width: 2px; border: 1px solid black;display:inline;background-color:black;"></div>
			{user.role.toUpperCase()}
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
				<li>
					<button onclick={logout}> Logout </button>
				</li>
				<!-- <li><a href="/"><span class="material-symbols-outlined"> person </span></a></li> -->
			</ul>
		</nav>
	</div>
</section>

<style>
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
		background-color: white;
		border: 1px solid rgb(211, 211, 211);
		border-radius: 5px;
		padding: 1rem;
		z-index: 1;
		width: 15rem;
		transform: translateX(-35%);
	}
	.material-symbols-outlined {
		font-variation-settings:
			'FILL' 1,
			'wght' 400,
			'GRAD' 0,
			'opsz' 24;
	}
	nav {
		margin-left: auto;
	}

	a {
		text-decoration: none;
		color: black;
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
