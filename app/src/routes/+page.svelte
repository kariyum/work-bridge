<script lang="ts">
	import LandingPage from '$lib/pages/LandingPage.svelte';
	import Projects from '$lib/components/project/Projects.svelte';
	let { data } = $props();
</script>

{#if !data.user}
	<LandingPage></LandingPage>
{:else if data.error}
	<h1>You seem to be offline...</h1>
{:else}
	{@const projects = data.projects}
	<div class="page-container">
		<Projects {projects}></Projects>
	</div>
{/if}