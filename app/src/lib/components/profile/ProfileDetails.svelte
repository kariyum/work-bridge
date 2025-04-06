<script lang="ts">
	import { User } from 'lucide-svelte';
	import type { ProfileGET } from '../../../routes/settings/+layout';

	type Props = {
		profile: ProfileGET;
	};
	let { profile }: Props = $props();
	let links = $derived([profile.github_link, profile.linkedin_link, profile.portfolio_link]);
</script>

<div class="container">
	<div>
		<p>{profile.role}</p>
		<h1>{profile.first_name} {profile.last_name}</h1>
	</div>
	<div>
		<h4>Bio</h4>
		{#if !profile.bio || profile.bio?.length == 0}
			<p>Edit to add a bio.</p>
		{:else}
			<p>
				{profile.bio}
			</p>
		{/if}
	</div>
	{#if profile.role === 'freelancer'}
		<div class="links">
			<h4>Links</h4>
			{#if links.every((value) => !value)}
				Edit to add links.
			{:else}
				{#if profile.linkedin_link}
					<a href={profile.linkedin_link}>LinkedIn</a>
				{/if}
				{#if profile.portfolio_link}
					<a href={profile.portfolio_link}>Portfolio Website</a>
				{/if}
			{/if}
		</div>
		<div>
			<h4>Skills</h4>
			{#if !profile.skills || profile.skills?.length === 0}
				<div>Edit to add skills.</div>
			{:else}
				<div class="skills">
					{#each profile.skills as skill}
						<div class="skill">{skill}</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	a {
		width: fit-content;
	}
	.links {
		display: flex;
		flex-direction: column;
	}

	.container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.skills {
		display: flex;
		margin-top: 0.5rem;
		gap: 0.5rem;
	}

	.skill {
		background-color: var(--tag-bg);
		padding: 0.5rem 0.8rem;
		align-items: center;
		border-radius: 5px;
		width: fit-content;
		line-height: 1rem;
	}
</style>
