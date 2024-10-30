<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import './styles.css';
	import { userStore } from '$lib/storage';
	let { data, children } = $props();
</script>

{#if $userStore}
	<Navbar user={$userStore} />
	{@render children()}
{:else}
	{@render children()}
{/if}
