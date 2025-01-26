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
				<label for="name">Name</label>
				<input type="text" id="name" name="name" />
			</div>

			<div class="input-group">
				<label for="description">Tell us about yourself</label>
				<textarea id="description" name="description"
				></textarea>
			</div>
			<div class="input-group">
				<label for="skills">Skills</label>
				<div class="skills">
					<Skills skillsIn={skills} {addSkill} {removeSkillAtIndex}></Skills>
				</div>
			</div>
			<div class="input-group">
				<label for="dob">Date of Birth</label>
				<input type="date" id="dob" name="birthdate" />
			</div>
			<div class="input-group">
				<label for="phone_number">Phone Number</label>
				<input type="tel" name="phone" id="phone_number" />
			</div>
			<input type="submit" value="Save" />
		</form>
	</div>
</div>

<style>
	.container {
		width: fit-content;
	}
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

	input[type="submit"] {
		cursor: pointer;
	}
	
	.skills:focus {
		outline: 2px solid var(--blue);
		outline-offset: -2px;
	}
</style>
