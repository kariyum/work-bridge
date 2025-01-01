<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import './styles.css';
	import { onMount } from "svelte";
	let { data, children } = $props();	

	let theme = $state("light");

	const toggleTheme = () => {
		theme = theme === "light" ? "dark" : "light";
		document.documentElement.setAttribute("data-theme", theme);
		localStorage.setItem("theme", theme);
	};

	onMount(() => {
		const savedTheme = localStorage.getItem("theme") || "light";
		theme = savedTheme;
		document.documentElement.setAttribute("data-theme", theme);
	});

</script>
<button class="theme-button" onclick={toggleTheme}>
	Switch to {theme === "light" ? "Dark" : "Light"} Mode
  </button>
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
	.theme-button {
	background-color: var(--btn-bg);
	color: var(--dark-text);
	border: 2px solid var(--border);
	border-radius: 50px;
	padding: 12px 24px;
	cursor: pointer;
	transition: background-color 0.3s ease, color 0.3s ease;
	}

	.container {
		margin: auto;
		max-width: 1300px;
		padding: 0 1rem;
	}
</style>
