<script lang="ts">
	import CreateProject from '$lib/components/CreateProject.svelte';

	const { data, children } = $props();
</script>

{#key data}
	<CreateProject projectIn={data.project}></CreateProject>
{/key}

{@render children()}
