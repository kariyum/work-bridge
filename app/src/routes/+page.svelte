<script lang="ts">
	import LandingPage from '$lib/pages/LandingPage.svelte';
	import LandingPageFreelancer from '$lib/pages/LandingPageFreelancer.svelte';
	import LandingPageRecruiter from '$lib/pages/LandingPageRecruiter.svelte';
	import { userStore } from '$lib/storage.js';
	let { data } = $props();
	if (data.status === 401) {
		userStore.set(undefined);
	}
</script>

{#await data.projects}
	<h1>Fetching...</h1>
{:then projects}
	{#if $userStore?.role === 'recruiter'}
		<LandingPageRecruiter {projects}></LandingPageRecruiter>
	{:else if $userStore?.role === 'freelancer'}
		<LandingPageFreelancer {projects}></LandingPageFreelancer>
	{:else}
		<LandingPage></LandingPage>
	{/if}
{/await}
