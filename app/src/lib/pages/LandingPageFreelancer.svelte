<script lang="ts">
	import ProjectFreelancer from "$lib/components/ProjectFreelancer.svelte";
	import type { ProjectGET } from "$lib/types/project";
    let { projects }: {projects: ProjectGET[]} = $props();
    let pageTitle = "Landing Page Freelancer";
</script>

<svelte:head>
    <title>{pageTitle}</title>
</svelte:head>

<div class="outer-container">
    <h2>Projects Feed</h2>
    {#each projects as project}
        <ProjectFreelancer {project} />
    {/each}
</div>


<style>

</style>