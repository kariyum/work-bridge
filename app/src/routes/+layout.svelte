<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
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
	<div class="container">
		<Navbar user={data.user} />
	</div>
	<div class="container">
		{@render children()}
	</div>
{:else}
	{@render children()}
{/if}

<style>
	.container {
		margin: auto;
		max-width: 1300px;
		padding: 0 1rem;
	}
</style>
