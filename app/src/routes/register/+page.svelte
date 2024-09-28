<script>
	import { goto } from '$app/navigation';
	/** @type { HTMLFormElement } */
	let formElement;
	/**
	 * @type {Map<string, string>}
	 */
	let errorMessages = new Map([
		['email', 'Please enter a valid email'],
		['password', 'Please enter a valid password'],
		['confirm_password', 'Passwords do not match']
	]);
	let error_message = '';
	async function register() {
		error_message = '';
		let formData = new FormData(formElement);
		console.log('Form Data', formData.entries());
		let invalid = false;
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
		console.log(invalid);
		if (invalid) return;
		let data = new URLSearchParams(
			Array.from(formData.entries()).map(([key, value]) => [key, value.toString()])
		);
		const response = await fetch('/api/register', {
			method: 'POST',
			body: data
		});
		if (response.ok) { 
			// wasn't able to check if the cookie was indeed set here
			// the sveltkit server on the other hand was able to check on the cookie
			// because it's set http only
			goto('/');
		}
	}

	async function getUsers() {
		const response = await fetch('/api/users');
		const data = await response.json();
		console.log(data);
	}
</script>

<button on:click={getUsers}> Get users </button>

<div class="container">
	<h1>Register</h1>
	<form on:submit|preventDefault={register} bind:this={formElement}>
		<p>
			{error_message}
		</p>
		<input type="text" name="email" id="email" placeholder="Email" required />
		<input type="password" name="password" id="password" placeholder="Password" required />
		<input
			type="password"
			name="confirm_password"
			id="confirm_password"
			placeholder="Confirm password"
		/>
		<input type="text" name="name" id="name" placeholder="name" required />
		<input type="text" name="surname" id="surname" placeholder="surname" required />
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
		<button type="submit">Register</button>
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
		/* align-items: center; */
		flex-wrap: wrap;
		width: 30%;
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
