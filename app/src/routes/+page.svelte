<script lang="ts">
	import LandingPage from '$lib/pages/LandingPage.svelte';
	import LandingPageFreelancer from '$lib/pages/LandingPageFreelancer.svelte';
	import LandingPageRecruiter from '$lib/pages/LandingPageRecruiter.svelte';
	let { data } = $props();
</script>

{#if data.error && data.error?.unauthorizedError}
	<LandingPage></LandingPage>
{:else if data.error && data.error?.networkError}
	<h1>You seem to be offline...</h1>
{:else}
	{@const projects = data.projects}
	<div class="container">
		{#if data.user?.role === 'recruiter'}
			<LandingPageRecruiter {projects}></LandingPageRecruiter>
		{:else if data.user?.role === 'freelancer'}
			<LandingPageFreelancer {projects}></LandingPageFreelancer>
		{/if}
	</div>
{/if}


<style>
	.container {
		margin-top: 1rem;
	}
</style>