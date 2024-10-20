<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/storage';
	import { cyrb53, validateEmail } from '$lib/utils.js';

	let formElement: HTMLFormElement;
	let errorMessages: Map<string, string> = new Map([
		['email', 'Please enter a valid email'],
		['password', 'Please enter a valid password'],
		['confirm_password', 'Passwords do not match']
	]);
	let error_message: string = '';
	
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
			console.log(key);
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
		const response = await fetch('/api/register', {
			method: 'POST',
			body: data
		});
		if (response.ok) {
			authStore.set(true);
			goto('/');
		}
	}
</script>

<div class="container">
	<h1>Register</h1>
	<form on:submit|preventDefault={register} bind:this={formElement}>
		<p>
			{error_message}
		</p>
		<input type="email" name="email" id="email" placeholder="Email" required />
		<input type="password" name="password" id="password" placeholder="Password" required />
		<input
			type="password"
			name="confirm_password"
			id="confirm_password"
			placeholder="Confirm password"
		/>
		<input type="text" name="first_name" id="first_name" placeholder="Name" required />
		<input type="text" name="last_name" id="last_name" placeholder="Last Name" required />
		<div class="options" id="role">
			<span>
				<input type="radio" id="recruiter" name="role" value="recruiter" required checked />
				<label for="recruiter">Recruiter</label>
			</span>
			<span>
				<input type="radio" id="freelancer" name="role" value="freelancer" required />
				<label for="freelancer">Freelancer</label>
			</span>
		</div>
		<div style="display: flex; justify-content: space-between; width: 100%;">
			<a href="/login">
				<button style="width: 100%;">
					Login
				</button>
			</a>
			<button type="submit">Register</button>
		</div>
	</form>
</div>

<style>
	.options {
		display: flex;
		justify-content: space-between;
		width: 100%;
		flex-wrap: wrap;
	}
	.container {
		display: flex;
		justify-content: space-between;
		align-items: safe center;
		flex-wrap: wrap;
		width: 60%;
		padding: 10%;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}
	form {
		display: flex;
		flex-direction: column;
		align-items: baseline;
		width: min-content;
		gap: 10px;
		width: 50%;
	}
</style>
