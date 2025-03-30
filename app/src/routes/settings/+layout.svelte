<script lang="ts">
	import { Settings, User } from 'lucide-svelte';

	let { children, data } = $props();
    let onPage = $derived(data.url);
    const pageToHeader = new Map([
        ["profile", "Public profile"],
        ["account", "Account settings"]
    ]);
</script>

<div class="outer-container">
	<div>
		{#if data.user}
			<h2>{data.user.email}</h2>
			<p>Your personal account</p>
		{/if}
	</div>
	<div class="container">
		<div class="menu">
			<a href="/settings/profile" data-selected={onPage === "profile"}> <User /> Public profile </a>
			<a href="/settings/account" data-selected={onPage === "account"}> <Settings /> Account Settings </a>
		</div>
		<div class="main">
            <h2>{pageToHeader.get(onPage) ?? "NOT FOUND"}</h2>
            <hr>
			{@render children()}
		</div>
	</div>
</div>

<style>
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

    a[data-selected = "true"] {
        background-color: var(--selected-color);
    }

    a[data-selected = "true"]::before {
        content: "";
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

	.container {
		display: grid;
		grid-template-columns: 30ch 1fr;
        gap: 3rem;
        margin-top: 1rem;
	}

	.menu {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
