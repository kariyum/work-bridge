<script lang="ts">
	import Navbar from '$lib/components/navbar/Navbar.svelte';
	import NavbarNewUser from '$lib/components/navbar/NavbarNewUser.svelte';
	import './styles.css';
	let { data, children } = $props();
</script>

<svelte:head>
	<script>
		document.documentElement.classList.add(
			localStorage.getItem('app:theme') ??
				(window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
		);
	</script>
</svelte:head>
{#if data.user}
	<Navbar user={data.user} notifications={data.notifications} />
	<div class="container">
		{@render children()}
	</div>
{:else}
	<NavbarNewUser></NavbarNewUser>
	{@render children()}
{/if}

<style>
	.container {
		margin: auto;
		max-width: var(--max-width);
	}
</style>
