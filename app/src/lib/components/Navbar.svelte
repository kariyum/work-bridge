<script lang="ts">
	import { goto } from '$app/navigation';
	import { userStore } from '$lib/storage';
	let { user }: { user: User } = $props();
	async function logout() {
		const response = await fetch('/api/logout');

		if (response.status === 200) {
			userStore.set(undefined);
			await goto('/');
		}
	}

	let showNotifications = $state(false);
	let dropdownModal: HTMLDivElement;

	function notificationClickHandler(event: MouseEvent) {
		if ((event.target as Element)?.closest('.notifications')) {
			showNotifications = true;
		} else {
			showNotifications = false;
		}
	}
</script>

<svelte:body onclick={notificationClickHandler} />
<section>
	<div class="container">
		<h1><a href="/">Word-bridge</a> | {$userStore?.role.toUpperCase()}</h1>
		<nav>
			<ul>
				<!-- <li><a href="/"><span class="material-symbols-outlined">home</span></a></li> -->
				<li><a href="/profile">(Avatar) {user.email}</a></li>
				<li><a href="/project">Create a project</a></li>
				<li><a href="/messages">Discussions</a></li>
				<li class="notifications">
					<button>Notifications</button>
					<div class="notification-container" class:showNotifications bind:this={dropdownModal}>
						<h1>Notifications</h1>
						<div>Notification 1</div>
						<div>Notification 2</div>
					</div>
				</li>
				<li><a href="/settings">Settings</a></li>
				<li>
					<button onclick={logout}>Logout</button>
				</li>
				<!-- <li><a href="/"><span class="material-symbols-outlined"> person </span></a></li> -->
			</ul>
		</nav>
	</div>
</section>

<style>
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
		width: 40%;
		margin-left: auto;
	}
	.container {
		/* border: 2px solid black; */
		display: flex;
		flex-direction: row nowrap;
		align-items: safe center;
		height: max-content;
		margin: 1rem;
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
	}
</style>
