<script lang="ts">
	import CreateProject from '$lib/components/CreateProject.svelte';
	import ProjectDetails from '$lib/components/ProjectDetails.svelte';

	const { data, children } = $props();

	let tasksGlobalState = $derived.by(() => {
		let state = $state(data.tasksGlobalState);
		return state;
	});
	let editMode = $state(false);
</script>

{#if data.project !== undefined}
	<ProjectDetails
		projectIn={data.project}
		role={data.user?.role ?? ''}
		userId={data.user?.email ?? ''}
		onedit={() => (editMode = true)}
	/>
{:else if editMode || data.user?.role === 'recruiter'}
	<CreateProject projectIn={data.project} {tasksGlobalState}></CreateProject>
{:else}
	Project does not exist
{/if}

{@render children()}
