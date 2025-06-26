<script lang="ts">
	import Projects from '$lib/components/project/Projects.svelte';
	let { data } = $props();
</script>

{#if data.error}
	We encountered an error {data.error}
{:else}
	<div class="page-container">
		<Projects projects={data.projects}></Projects>
	</div>
{/if}
