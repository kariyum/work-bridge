<script lang="ts">
	import LandingPage from '$lib/pages/LandingPage.svelte';
	import LandingPageFreelancer from '$lib/pages/LandingPageFreelancer.svelte';
	import LandingPageRecruiter from '$lib/pages/LandingPageRecruiter.svelte';
	let { data } = $props();
</script>

{#await data.result}
	<h1>Fetching...</h1>
{:then value}
	{#if value.error}
		<h1>There was an error, please try again...</h1>
	{:else}
		{@const projects = value.projects}
		{#if data.user?.role === 'recruiter'}
			<LandingPageRecruiter {projects}></LandingPageRecruiter>
		{:else if data.user?.role === 'freelancer'}
			<LandingPageFreelancer {projects}></LandingPageFreelancer>
		{:else}
			<LandingPage></LandingPage>
		{/if}
	{/if}
{/await}
