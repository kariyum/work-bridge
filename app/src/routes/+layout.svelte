<script context="module">
	import Navbar from '$lib/components/Navbar.svelte';
	import { authStore } from '$lib/storage';
	import './styles.css';
</script>

{#if $authStore}
	<Navbar />
	<slot />
{:else}
	<slot />
{/if}
