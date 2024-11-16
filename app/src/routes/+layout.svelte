<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import './styles.css';
	import { userStore } from '$lib/storage';
	let { data, children } = $props();
</script>

{#if $userStore}
	<div class="container">
		<Navbar user={$userStore} />
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
		max-width: 1500px;
	}
</style>
