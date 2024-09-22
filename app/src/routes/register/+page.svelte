<script>
	import { goto } from '$app/navigation';

	let email = '';
	let password = '';
	let confirmPassword = '';

	async function register() {
		if (!email || !password || !confirmPassword) {
			alert('Please fill in all fields');
			return;
		}
		if (password !== confirmPassword) {
			alert('Passwords do not match');
			return;
		}
		const response = await fetch('/api/register', {
			method: 'POST',
			body: new URLSearchParams({
				email: email,
				password: password,
                confirmPassword: confirmPassword
			})
		});
		if (response.ok) {
			goto('/');
		}
	}

	async function getUsers() {
		const response = await fetch('/api/users');
		const data = await response.json();
		console.log(data);
	}
</script>

<button on:click={getUsers}>
	Get users
</button>

<div class="container">
	<h1>Register</h1>
	<form on:submit|preventDefault={register}>
		<input type="text" name="email" placeholder="Email" required bind:value={email} />
		<input type="password" name="password" placeholder="Password" required bind:value={password} />
		<input
			type="password"
			name="confirm-password"
			placeholder="Confirm password"
			bind:value={confirmPassword}
		/>
		<button type="submit">Register</button>
	</form>
</div>

<style>
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
