<script lang="ts">
	import { pushState } from '$app/navigation';
	import { page } from '$app/state';
	import CreateProject from '$lib/components/CreateProject.svelte';
	import ProjectDetails from '$lib/components/ProjectDetails.svelte';

	const { data } = $props();
</script>

{#if data.project !== undefined}
	{#if page.state.projectEditMode ?? false}
		<CreateProject projectIn={data.project} />
	{:else}
		<ProjectDetails
			projectIn={data.project}
			role={data.user?.role ?? ''}
			userId={data.user?.email ?? ''}
			onEdit={() => {
				pushState('', {
					projectEditMode: true,
					showTaskPopup: false
				});
			}}
		/>
	{/if}
{:else}
	Project does not exist
{/if}
