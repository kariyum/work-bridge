<script lang="ts">
	import { pushState } from '$app/navigation';
	import { page } from '$app/state';
	import CreateProject from '$lib/components/project/CreateProject.svelte';
	import ProjectDetails from '$lib/components/project/ProjectDetails.svelte';

	const { data } = $props();
</script>

{#if data.project !== undefined}
	{#if page.state.projectEditMode ?? false}
		<CreateProject projectIn={data.project} />
	{:else}
		<ProjectDetails
			projectIn={data.project}
			user={data.user}
			onEdit={() => {
				pushState('', {
					projectEditMode: true,
					showTaskPopup: false,
					profileEditMode: false
				});
			}}
		/>
	{/if}
{:else}
	Project does not exist
{/if}
