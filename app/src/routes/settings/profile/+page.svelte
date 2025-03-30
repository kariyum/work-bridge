<script lang="ts">
	import Skills from '$lib/components/Skills.svelte';

	let { data } = $props();
	let skills: string[] = $state([]);
	const addSkill = (skill: string) => skills.push(skill);
	const removeSkillAtIndex = (index: number) => skills.splice(index, 1);
	let form: HTMLFormElement;
	async function saveProfile(event: SubmitEvent) {
		event.preventDefault();
		const formData = new FormData(form);
		const entries = formData.entries().filter(([key, value]) => key !== 'first_name');
		const profile = Object.fromEntries(entries);
		const payload = {
			...profile,
			skills,
			birthdate: new Date(profile.birthdate as string).toISOString()
		};
		const response = await fetch('/api/profile', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});
		if (!response.ok) {
			console.error('/api/profile failed');
		}
	}
</script>

<div class="top-container">
	<div class="container">
		<form bind:this={form} onsubmit={saveProfile}>
			<div class="input-group">
				<label for="first_name">First Name</label>
				<input type="text" id="first_name" name="first_name" />
			</div>
			<div class="input-group">
				<label for="last_name">Last Name</label>
				<input type="text" id="last_name" name="last_name" />
			</div>

			<div class="input-group">
				<label for="description">Tell us about yourself</label>
				<textarea id="description" name="description"></textarea>
			</div>
			{#if data.user?.role == 'freelancer'}
				<div class="input-group">
					<label for="linkedin_profile">LinkedIn Profile</label>
					<input type="text" id="linkedin_profile" name="linkedin_link">
				</div>
				<div class="input-group">
					<label for="portfolio">Personal Website</label>
					<input type="text" id="portfolio" name="portfolio">
				</div>
				<div class="input-group">
					<label for="skills">Skills</label>
					<div class="skills">
						<Skills skillsIn={skills} {addSkill} {removeSkillAtIndex}></Skills>
					</div>
				</div>
			{/if}
			<div>
				<input type="submit" value="Save" />
			</div>
		</form>
	</div>
</div>

<style>

	.top-container {
		width: 100%;
	}
	.skills {
		width: 30rem;
	}
	label {
		width: fit-content;
	}
	form {
		margin-top: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
	.input-group {
		display: flex;
		gap: 0.3rem;
		flex-direction: column;
	}

	input,
	textarea {
		border: 2px solid var(--border);
	}

	input[type='submit'] {
		display: block;
		cursor: pointer;
		margin-left: auto;
		background-color: var(--btn-bg);

		&:hover {
			background-color: var(--hover-color);
		}
	}

	.skills:focus {
		outline: 2px solid var(--blue);
		outline-offset: -2px;
	}
</style>
