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

{#if $userStore?.role === 'recruiter'}
	<LandingPageRecruiter projects={data.projects}></LandingPageRecruiter>
{:else if $userStore?.role === 'freelancer'}
	<LandingPageFreelancer projects={data.projects}></LandingPageFreelancer>
{:else}
	<LandingPage></LandingPage>
{/if}
