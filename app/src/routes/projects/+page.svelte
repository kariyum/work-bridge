<script lang="ts">
	import Projects from '$lib/components/project/Projects.svelte';
	let { data } = $props();
</script>

{#if data.error}
	We encountered an error {data.error}
{:else}
	<div class="container">
		<Projects projects={data.projects}></Projects>
	</div>
{/if}

<style>
	.container {
		padding: 1rem;
		margin: auto;
		max-width: var(--max-width);
	}
</style>
