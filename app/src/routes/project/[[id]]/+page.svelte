<script lang="ts">
	import CreateProject from "$lib/components/CreateProject.svelte";

    const { data } = $props();
    console.log(data);
</script>

<CreateProject project={data.project} ></CreateProject>