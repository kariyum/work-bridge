<script lang="ts">
	import ProjectFreelancer from "$lib/components/ProjectFreelancer.svelte";

    let { projects }: {projects: ProjectGET[]} = $props();
</script>

<div>
    <h1>Projects</h1>
    {#each projects as project}
        <ProjectFreelancer {project} />
    {/each}
</div>