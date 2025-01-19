<script lang="ts">
	import { goto } from '$app/navigation';
	import { cyrb53 } from '$lib/utils.js';
	import { MoveLeft } from 'lucide-svelte';

	let formElement: HTMLFormElement;
	let error_message: string = $state('');
	let formData: Record<string, string> = $state({ role: '' });

	let isRoleSelected = $derived(formData.role !== '');
	let continueSelected = $state(false);
	function handleRadioChange(event: Event) {
		const target = event.target as HTMLInputElement;
		if (target) formData.role = target.value;
	}

	function captureFormData() {
		const _formData = new FormData(formElement);
		const formValues = Object.fromEntries(
			_formData.entries().map(([key, value]) => [key, String(value)])
		) as Record<string, string>;
		formData = { ...formData, ...formValues };
	}

	function processFormData() {
		const hashedPassword = cyrb53(formData.password ?? '').toString();
		const obj = {
			...formData,
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

	async function handleSubmit(event: Event) {
		event.preventDefault();
		captureFormData();

		if (formData.role.length !== 0) {
			await sendRequest();
		}
	}
	
	function redirectToHomePage() {
		goto('/');
	}

	function goToNextStep() {
		captureFormData();
		if (!isRoleSelected) {
			formData.role = formData.role || '';
			return;
		}
		continueSelected = !continueSelected;
	}
</script>

<div class="container">
	<div class="sub-container">
		<a href="/" data-testid="home-link" title="Go back home">
			<MoveLeft size="3rem" />
		</a>
		<h1>Join us!</h1>
		<form onsubmit={handleSubmit} bind:this={formElement} data-testid="register-form">
			{#if !continueSelected}
				<h2>I'm a ...</h2>
				<div class="role-picker-container">
					<div class="options" id="role">
						<label class="card" data-testid="role-recruiter" for="recruiter">
							<input
								type="radio"
								id="recruiter"
								name="role"
								onchange={handleRadioChange}
								value="recruiter"
								required
								checked={formData.role === 'recruiter'}
							/>
							<div class="card-body">
								<h1>Recruiter</h1>
								<p>I have a project, I need freelancers.</p>
							</div>
						</label>
						<label class="card" data-testid="role-freelancer" for="freelancer">
							<input
								type="radio"
								id="freelancer"
								name="role"
								onchange={handleRadioChange}
								value="freelancer"
								required
								checked={formData.role === 'freelancer'}
							/>
							<div class="card-body">
								<h1>Freelancer</h1>
								<p>I have the skills, looking for work.</p>
							</div>
						</label>
					</div>
				</div>
			{:else}
				<h2>Tell us about you!</h2>
				<div class="personal-info-container">
					<p data-testid="error-message">{error_message}</p>
					<div class="username">
						<input type="text" name="first_name" id="first_name" placeholder="Name" required data-testid="first-name" />
						<input type="text" name="last_name" id="last_name" placeholder="Last Name" required data-testid="last-name" />
					</div>
					<p class="input-info">
						We use your name and last name to display them throughout the app, (e.g. on your posts,
						discussions and profile)
					</p>
					<input type="email" name="email" id="email" placeholder="Email" required data-testid="email" />
					<input type="password" name="password" id="password" placeholder="Password" required data-testid="password" />
					<input
						type="password"
						name="confirm_password"
						id="confirm_password"
						placeholder="Confirm password"
						data-testid="confirm-password"
					/>
				</div>
			{/if}

			<div class="actions">
				<a href="/login" data-testid="login-link">Already have an account? Login!</a>
				<div style="margin-left:auto;">
					<button type="button" onclick={() => {
						if (formData.role.length == 0) redirectToHomePage();
						formData.role = ''; continueSelected = !continueSelected;
					}} data-testid="previous-button">
						Back
					</button>
					{#if !continueSelected}
						<button type="button" onclick={goToNextStep}>
							Continue
						</button>
					{:else}
						<button type="submit">Submit</button>
					{/if}
				</div>
			</div>
		</form>
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
		border: 1px solid #ccc;
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
	.card:has(input:checked)#recruiter {
		/* scale: 1.05; */
		background-color: var(--orange);
		box-shadow: 0 0 1rem var(--orange);
		border-color: var(--orange);
	}

	.card:hover[for='freelancer'],
	.card:has(input:checked)#freelancer {
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