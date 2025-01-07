<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import './styles.css';
	import { onMount } from "svelte";
	let { data, children } = $props();	

	let theme = $state("light");



</script>

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
