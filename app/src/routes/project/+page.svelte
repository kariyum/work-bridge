<script lang="ts">
	import CreateProject from '$lib/components/CreateProject.svelte';

	const { data } = $props();

	let tasksGlobalState = $derived.by(() => {
		let state = $state(data.tasksGlobalState);
		return state;
	});
</script>

<CreateProject {tasksGlobalState}></CreateProject>