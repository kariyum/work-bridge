<script lang="ts">
	import { page } from '$app/state';
	import { Settings, User } from 'lucide-svelte';

	let { children, data } = $props();
	let onPage = $derived(page.url.pathname.slice(1).split('/')[1]);
	const pageToHeader = new Map([
		['profile', 'Public Profile'],
		['account', 'Account Settings']
	]);
</script>

<div class="outer-container">
	<a href="/settings" style="padding: 0"><h1>Settings</h1></a>
	<p style="color:var(--sub-title)">Manage your account settings and preferences.</p>
	<div class="container">
		<div class="menu" data-selected={pageToHeader.get(onPage) != undefined}>
			<a class="link" href="/settings/profile" data-selected={onPage === 'profile'}>
				<User />
				<div>Public Profile</div>
			</a>
			<a class="link" href="/settings/account" data-selected={onPage === 'account'}>
				<Settings /> Security & Login
			</a>
		</div>
		<div class="main card card-padding" data-selected={pageToHeader.get(onPage) != undefined}>
			<!-- {#if pageToHeader.get(onPage)}
				<h2>{pageToHeader.get(onPage)}</h2>
			{/if} -->
			{@render children()}
		</div>
	</div>
</div>

<style>
	.outer-container {
		max-width: var(--page-width);
		margin: 0.5rem auto;
		padding: 1rem;
		padding-top: 0;
	}

	a {
		text-decoration: none;
		background-color: transparent;
		color: var(--font-color);
	}

	.link {
		position: relative;
		gap: 1rem;
		display: flex;
		border: 2px solid transparent;
		padding: 0.5rem;
		border-radius: 3px;
		align-items: center;
		font-size: large;
	}

	.link[data-selected='true'] {
		background-color: var(--selected-color);
	}

	.link[data-selected='true']::before {
		content: '';
		position: absolute;
		width: 5px;
		height: 2.2rem;
		border-radius: 10px;
		background-color: var(--blue);
		left: -0.6rem;
		top: 0.15rem;
	}

	.link:hover {
		background-color: var(--hover-color);
	}

	.menu {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	@media (width < 600px) {
		.menu[data-selected='true'] {
			display: none;
		}

		.main[data-selected='false'] {
			display: none;
		}
		.outer-container {
			padding: 1rem;
			padding-top: 0;
		}
		.menu {
			width: 100%;
			flex-grow: 1;
			a {
				width: 100%;
			}
		}
		.container {
			margin-top: 1rem;
			width: 100%;
		}
	}

	@media (width >= 600px) {
		.main[data-selected='false'] {
			display: none;
		}
		.container {
			display: grid;
			grid-template-columns: 30ch 1fr;
			gap: 3rem;
			margin-top: 1rem;
		}
	}
</style>
