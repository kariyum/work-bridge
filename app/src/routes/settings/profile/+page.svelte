<script lang="ts">
	import { goto, pushState } from '$app/navigation';
	import { page } from '$app/state';
	import CreateProfile from '$lib/components/profile/CreateProfile.svelte';
	import ProfileDetails from '$lib/components/profile/ProfileDetails.svelte';
	import { onMount } from 'svelte';

	let { data } = $props();

	onMount(async () => {
		if (data.error?.unauthorizedError) {
			await goto('/', { invalidateAll: true });
		}
	});

	const switchToProfileEditMode = () => {
		pushState('', {
			profileEditMode: true,
			projectEditMode: false,
			showTaskPopup: false
		});
	};
</script>

{#if data.profileData.error?.parsingError}
	<p>It's not you, it's us. Parsing error. {data.profileData.error.parsingError.message}</p>
{:else}
	<div class="page">
		{#if data.profileData.isOk() && page.state.profileEditMode}
			<CreateProfile user={data.profileData.getOrThrow()} />
		{:else}
			<div class="edit-profile">
				<ProfileDetails profile={data.profileData.getOrThrow()} />
				<button onclick={switchToProfileEditMode}>Edit Profile</button>
			</div>
		{/if}
	</div>
{/if}

<style>
	.edit-profile {
		display: flex;
		align-items: start;
		justify-content: space-between;

		> button {
			margin-bottom: auto;
		}
	}

	.page {
		margin-top: 1rem;
	}
</style>
