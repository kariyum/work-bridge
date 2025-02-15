<script lang="ts">
	import CreateProject from '$lib/components/CreateProject.svelte';
	import ProjectDetails from '$lib/components/ProjectDetails.svelte';

	const { data } = $props();

</script>

{#if data.project !== undefined}
	<ProjectDetails
		projectIn={data.project}
		role={data.user?.role ?? ''}
		userId={data.user?.email ?? ''}
	/>
{:else}
	Project does not exist
{/if}
