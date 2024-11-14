<script lang="ts">
	import CreateProject from '$lib/components/CreateProject.svelte';

	const { data, children } = $props();
</script>

{#key data}
	<CreateProject
		projectId={data.project?.id}
		title={data.project?.title}
		content={data.project?.content}
		budget={data.project?.budget}
		deadline={data.project?.deadline}
	></CreateProject>
{/key}

{@render children()}