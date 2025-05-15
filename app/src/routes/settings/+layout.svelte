<script lang="ts">
	import { page } from '$app/state';
	import { Settings, User } from 'lucide-svelte';

	let { children, data } = $props();
	let onPage = $derived(page.url.pathname.slice(1).split('/')[1]);
	const pageToHeader = new Map([
		['profile', 'Public profile'],
		['account', 'Account settings']
	]);
</script>

<div class="outer-container">
	<div>
		{#if data.user}
			<h2>{data.user.email}</h2>
		{/if}
	</div>
	<div class="container">
		<div class="menu" data-selected={pageToHeader.get(onPage) != undefined}>
			<a href="/settings/profile" data-selected={onPage === 'profile'}> <User /> Public profile </a>
			<a href="/settings/account" data-selected={onPage === 'account'}>
				<Settings /> Account Settings
			</a>
		</div>
		<div class="main" data-selected={pageToHeader.get(onPage) != undefined}>
			{#if pageToHeader.get(onPage)}
				<h2>{pageToHeader.get(onPage)}</h2>
				<hr />
			{/if}
			{@render children()}
		</div>
	</div>
</div>

<style>
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
		.container {
			display: grid;
			grid-template-columns: 30ch 1fr;
			gap: 3rem;
			margin-top: 1rem;
		}
	}
	hr {
		border-color: var(--border);
	}
	.outer-container {
		max-width: var(--page-width);
		margin: 0.5rem auto;
	}

	a {
		position: relative;
		gap: 1rem;
		display: flex;
		text-decoration: none;
		background-color: transparent;
		border: 2px solid transparent;
		color: var(--font-color);
		padding: 0.5rem;
		border-radius: 3px;
	}

	a[data-selected='true'] {
		background-color: var(--selected-color);
	}

	a[data-selected='true']::before {
		content: '';
		position: absolute;
		width: 5px;
		height: 2.2rem;
		border-radius: 10px;
		background-color: var(--blue);
		left: -0.6rem;
		top: 0.15rem;
	}

	a:hover {
		background-color: var(--hover-color);
	}

	.menu {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
