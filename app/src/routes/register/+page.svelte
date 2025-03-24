<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import { cyrb53, validateEmail } from '$lib/utils.js';
	import { Home, MoveLeft } from 'lucide-svelte';

	let formElement: HTMLFormElement;
	let errorMessages: Map<string, string> = new Map([
		['email', 'Please enter a valid email'],
		['password', 'Please enter a valid password'],
		['confirm_password', 'Passwords do not match']
	]);
	let error_message: string = $state('');

	async function register() {
		error_message = '';
		let formData = new FormData(formElement);
		let invalid = false;
		// check confirm_password and password
		const confirmPassword = formData.get('confirm_password')?.toString();
		const password = formData.get('password')?.toString();
		if (!password || !confirmPassword || confirmPassword !== password) {
			invalid = true;
		}
		const email = formData.get('email')?.toString();
		if (!email || !validateEmail(email)) {
			invalid = true;
		}
		formData.entries().forEach(([key, value]) => {
			invalid = invalid || !value;
			if (!value) {
				let element = document.getElementById(key);
				if (element) {
					element.style.border = '2px solid red';
				}
				error_message += errorMessages.get(key) + '\n';
			} else {
				let element = document.getElementById(key);
				if (element) {
					element.style.border = '';
				}
			}
		});
		if (invalid) return;
		formData.delete('confirm_password');
		const hashedPassword = cyrb53(password ?? '').toString();
		formData.set('password', hashedPassword);
		let data = new URLSearchParams(
			Array.from(formData.entries()).map(([key, value]) => [key, value.toString()])
		);
		const response = await fetch('/api/auth/register', {
			method: 'POST',
			body: data
		});
		if (response.ok) {
			await goto('/', { invalidateAll: true });
		}
	}
	function processFormData() {
		const hashedPassword = cyrb53(allData.password ?? '').toString();
		const obj = {
			...allData,
			password: hashedPassword
		};
		return Object.entries(obj).filter(([key, value]) => key !== 'confirm_password');
	}
	async function sendRequest() {
		const entries = processFormData();
		const payload = new URLSearchParams(Array.from(entries));
		const response = await fetch('/api/auth/register', {
			method: 'POST',
			body: payload
		});

		if (response.ok) {
			await goto('/', { invalidateAll: true });
		}
	}
	let allData: Record<string, string> = $state({});
	const steps = [rolePicker, form];
	let currentStep = $state(0);

	function captureFormData() {
		let formData = new FormData(formElement);
		let entries = Object.fromEntries(
			formData.entries().map(([key, value]) => [key, value.toString()])
		);

		allData = {
			...allData,
			...entries
		};
	}
</script>

{#snippet form()}
	<h2>Tell us about you!</h2>
	<div class="personal-info-container">
		<p>
			{error_message}
		</p>
		<div class="username">
			<input type="text" name="first_name" id="first_name" placeholder="Name" required />
			<input type="text" name="last_name" id="last_name" placeholder="Last Name" required />
		</div>
		<p class="input-info">
			We use your name and last name to display them throughout the app, (e.g. on your posts,
			discussions and profile)
		</p>
		<input type="email" name="email" id="email" placeholder="Email" required />
		<input type="password" name="password" id="password" placeholder="Password" required />
		<input
			type="password"
			name="confirm_password"
			id="confirm_password"
			placeholder="Confirm password"
		/>
	</div>
{/snippet}

{#snippet rolePicker()}
	<div class="role-picker-container">
		<h2>I'm a ...</h2>
		<div class="options" id="role">
			<label class="card" for="recruiter">
				<input
					type="radio"
					id="recruiter"
					name="role"
					value="recruiter"
					required
					checked={allData.role === 'recruiter'}
				/>
				<div class="card-body">
					<h1>Recruiter</h1>
					<p>I have a project, I need freelancers.</p>
				</div>
			</label>
			<label class="card" for="freelancer">
				<input
					type="radio"
					id="freelancer"
					name="role"
					value="freelancer"
					required
					checked={allData.role === 'freelancer'}
				/>
				<div class="card-body">
					<h1>Freelancer</h1>
					<p>I have the skills, looking for work.</p>
				</div>
			</label>
		</div>
	</div>
{/snippet}

<div class="container">
	<div class="sub-container">
		<a href="/" title="Go back home">
			<MoveLeft size="3rem" />
		</a>
		<h1>Join us!</h1>
		<form onsubmit={captureFormData} bind:this={formElement}>
			{@render steps[currentStep]()}
		</form>
		<div class="actions">
			{#if currentStep == 0}
				<a href="/login">Already have an account? Login!</a>
			{/if}
			<div style="margin-left:auto;">
				<button class="secondary-btn" onclick={() => (currentStep -= 1)} disabled={currentStep <= 0}
					>Previous</button
				>
				{#if currentStep < steps.length - 1}
					<button
						onclick={() => {
							captureFormData();
							currentStep += 1;
						}}
						disabled={currentStep >= steps.length - 1}>Continue</button
					>
				{:else}
					<button
						onclick={async () => {
							captureFormData();
							await sendRequest();
						}}>Submit</button
					>
				{/if}
			</div>
		</div>
	</div>
</div>

<style>
	.input-info {
		font-size: small;
		color: var(--grey);
	}
	.actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.username {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.5rem;
	}

	.personal-info-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.role-picker-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	input[type='radio'] {
		display: none;
	}

	.card-body {
		display: flex;
		flex-direction: column;
	}

	.card {
		border: 2px solid var(--border);
		padding: 5rem 2rem;
		border-radius: 5px;
		height: 100%;
		cursor: pointer;
		transition:
			background-color 0.2s ease-out,
			border-color 0.2s ease-out,
			box-shadow 0.2s ease-out;
	}

	.card:hover[for='recruiter'],
	.card[for='recruiter']:has(input:checked) {
		/* scale: 1.05; */
		background-color: var(--orange);
		box-shadow: 0 0 1rem var(--orange);
		border-color: var(--orange);
	}

	.card:hover[for='freelancer'],
	.card[for='freelancer']:has(input:checked) {
		/* scale: 1.05; */
		background-color: var(--blue);
		box-shadow: 0 0 1rem var(--blue);
		border-color: var(--blue);
	}

	.options {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.container {
		position: absolute;
		top: 5%;
		/* transform: translate(-50%, 0); */
		width: 100%;
	}

	a {
		color: var(--ucla-blue);
	}

	.sub-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 0 1rem;
		max-width: 45rem;
		margin: auto;
	}
</style>
