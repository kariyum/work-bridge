<script lang="ts">
	import CreateProject from '$lib/components/CreateProject.svelte';

	const { data, children } = $props();

	let tasksGlobalState = $derived.by(() => {
		let state = $state(data.tasksGlobalState);
		return state;
	});
</script>

{#key data}
	<CreateProject projectIn={data.project} {tasksGlobalState}></CreateProject>
{/key}

{@render children()}
